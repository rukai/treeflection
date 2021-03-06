use std::vec::Vec;
use std::ops::{Deref, DerefMut};
use serde::ser::{Serialize, Serializer};
use serde::de::{Deserialize, DeserializeOwned, Deserializer};
use serde_json;

use crate::node::Node;
use crate::node_runner::NodeRunner;
use crate::node_token::NodeToken;

#[derive(Clone, Default)]
pub struct ContextVec<T> {
    context: Vec<usize>,
    vector:  Vec<T>,
}

/// The purpose of a ContextVec is to provide a way for commands to easily access relevant values.
/// If we have a ContextVec named foo, the command `foo[?] get` will display the values in foo that the context points to.
///
/// # Contents
///
/// *   `vector: Vec<T>`
/// *   `context: Vec<usize>`
///
/// # Invariants
///
/// *   the values in context will point to a valid value in vector
/// *   the values in context will continue to point to the same value in vector (even after operations like insert and remove)

impl<T> ContextVec<T> {
    /// Create a new empty ContextVec
    pub fn new() -> ContextVec<T> {
        ContextVec {
            context: vec!(),
            vector:  vec!(),
        }
    }

    /// Create a new ContextVec from a Vec
    pub fn from_vec(vector: Vec<T>) -> ContextVec<T> {
        ContextVec {
            context: vec!(),
            vector: vector,
        }
    }

    /// Get the value currently pointed to by context
    pub fn selection_first(&self) -> Option<&T> {
        match self.context.first() {
            Some (value) => {
                self.vector.get(*value)
            }
            None => None
        }
    }

    /// Mutably get the value currently pointed to by context
    pub fn selection_first_mut(&mut self) -> Option<&mut T> {
        match self.context.first() {
            Some (value) => {
                self.vector.get_mut(*value)
            }
            None => None
        }
    }

    /// Get the values currently pointed to by context
    pub fn selection(&self) -> Vec<&T> {
        let mut result: Vec<&T> = vec!();
        for i in &self.context {
            result.push(&self.vector[*i]);
        }
        result
    }

    /// Get a slice of the context
    pub fn get_context(&self) -> &[usize] {
        &self.context
    }

    /// Consume the ContextVec into the context
    pub fn into_context(self) -> Vec<usize> {
        self.context
    }

    /// Consume the ContextVec into the vector
    pub fn into_vector(self) -> Vec<T> {
        self.vector
    }

    /// Consume the ContextVec into a tuple of the context and the vector
    pub fn into_tuple(self) -> (Vec<usize>, Vec<T>) {
        (self.context, self.vector)
    }

    /// Clears the context
    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    /// Sets a new context
    pub fn set_context(&mut self, value: usize) {
        let length = self.vector.len();
        if value >= length {
            panic!(format!("Attempted to set context {} on a ContextVec of length {}", value, length));
        }
        self.context.clear();
        self.context.push(value);
    }

    /// Sets a new context
    pub fn set_context_vec(&mut self, mut values: Vec<usize>) {
        self.context.clear();
        let length = self.vector.len();
        for value in values.drain(..) {
            if value >= length {
                panic!(format!("Attempted to set context {} on a ContextVec of length {}", value, length));
            }
            self.context.push(value);
        }
    }

    /// Set to a new vector.
    /// Clears the context.
    pub fn set_vec(&mut self, vector: Vec<T>) {
        self.context.clear();
        self.vector = vector;
    }

    /// Clears the vector and the context
    pub fn clear(&mut self) {
        self.context.clear();
        self.vector.clear();
    }

    /// Push a value to the end of the vector
    pub fn push(&mut self, value: T) {
        self.vector.push(value);
    }

    /// Insert a value into the vector.
    /// Invalid context indices are updated.
    pub fn insert(&mut self, index: usize, value: T) {
        self.vector.insert(index, value);

        for i in self.context.iter_mut() {
            if *i >= index {
                *i += 1;
            }
        }
    }

    /// Pop a value from the end of the vector.
    /// If it succeeds invalid context indices are removed.
    pub fn pop(&mut self) -> Option<T> {
        match self.vector.pop() {
            Some(value) => {
                let len = self.vector.len();
                self.context.retain(|&x| x < len);
                Some(value)
            }
            None => {
                None
            }
        }
    }

    /// Remove a value at the specified index.
    /// Deletes any context indices pointing to the removed value.
    /// Shifts all larger context indices down.
    pub fn remove(&mut self, to_remove: usize) -> T {
        let element = self.vector.remove(to_remove);
        let mut new_context: Vec<usize> = vec!();
        for i in self.context.drain(..) {
            if i < to_remove {
                new_context.push(i);
            }
            else if i > to_remove {
                new_context.push(i-1);
            }
        }
        self.context = new_context;
        element
    }
}

impl<T> Deref for ContextVec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        &self.vector
    }
}

impl<T> DerefMut for ContextVec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        &mut self.vector
    }
}

impl<T> Node for ContextVec<T> where T: Node + Serialize + DeserializeOwned + Default {
    fn node_step(&mut self, mut runner: NodeRunner) -> String {
        match runner.step() {
            NodeToken::ChainIndex (index) => {
                let length = self.vector.len();
                match self.vector.get_mut(index) {
                    Some (item) => item.node_step(runner),
                    None => {
                        return match length {
                             0 => format!("Used index {} on an empty vector", index),
                             1 => format!("Used index {} on a vector of size 1 (try 0)", index),
                             _ => format!("Used index {} on a vector of size {} (try a value between 0-{})", index, length, length-1)
                        }
                    }
                }
            }
            NodeToken::ChainContext => {
                let mut combined = String::from("|");
                for i in self.context.iter() {
                    match self.vector.get_mut(*i) {
                        Some(ref mut node) => {
                            let result = node.node_step(runner.clone());
                            combined.push_str(result.as_str());
                        }
                        None => {
                            combined.push_str("Context out of range. This should never happen.");
                        }
                    }
                    combined.push('|');
                }
                combined
            }
            NodeToken::ChainAll => {
                let mut combined = String::from("|");
                for item in self.vector.iter_mut() {
                    combined.push_str(item.node_step(runner.clone()).as_ref());
                    combined.push('|');
                }
                combined
            }
            NodeToken::ChainProperty (ref s) if s == "length" => { self.vector.len().node_step(runner) }
            NodeToken::Get => {
                serde_json::to_string_pretty(&mut self.vector).unwrap()
            }
            NodeToken::Set(value) => {
                match serde_json::from_str(&value) {
                    Ok(result) => {
                        self.vector = result;
                        String::from("")
                    }
                    Err(err) => {
                        format!("vector set error: {}", err)
                    }
                }
            }
            NodeToken::Insert => {
                self.push(T::default());
                String::new()
            }
            NodeToken::Remove => {
                if let Some(_) = self.pop() {
                    String::new()
                } else {
                    String::from("Tried to remove from an empty vector.")
                }
            }
            NodeToken::InsertIndex (index) => {
                let max_index = self.len();
                if index > max_index {
                    format!("Tried to insert at index {} on a vector of size {} (try a value between 0-{})", index, max_index, max_index)
                }
                else {
                    self.insert(index, T::default());
                    String::new()
                }
            }
            NodeToken::RemoveIndex (index) => {
                let max_index = self.len() - 1;
                if index > max_index {
                    format!("Tried to remove the value at index {} on a vector of size {} (try a value between 0-{})", index, self.len(), max_index)
                }
                else {
                    self.remove(index);
                    String::new()
                }
            }
            NodeToken::SetDefault => {
                self.vector = vec!();
                String::new()
            }
            NodeToken::Help => {
                String::from(r#"
Context Vector Help

Commands:
*   help          - display this help
*   get           - display JSON
*   set           - set to JSON
*   insert        - create a new element at the end of the vector
*   insert $INDEX - create a new element at $INDEX
*   remove        - remove the element at the end of the vector
*   remove $INDEX - remove the element at $INDEX
*   reset         - reset to empty vector

Accessors:
*   [INDEX] - access item at INDEX
*   [?]     - access items at current context
*   .length - display number of items"#)
            }
            action => { format!("vector cannot '{:?}'", action) }
        }
    }
}

impl<T> Serialize for ContextVec<T> where T: Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        self.vector.serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for ContextVec<T> where T: Deserialize<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        Ok(ContextVec {
            context: vec!(),
            vector: Vec::<T>::deserialize(deserializer)?,
        })
    }
}
