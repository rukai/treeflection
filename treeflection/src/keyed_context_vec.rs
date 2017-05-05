use std::iter::Zip;
use std::ops::{Deref, DerefMut, Index, IndexMut, Range, RangeTo, RangeFrom, RangeFull};
use std::slice::Iter;
use std::vec::Vec;

use itertools::join;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json;

use ::node::Node;
use ::node_runner::NodeRunner;
use ::node_token::NodeToken;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct KeyedContextVec<T> {
    #[serde(skip_serializing)]
    context: Vec<usize>,
    vector:  Vec<T>,
    keys:    Vec<String>,
}

/// A KeyedContextVec is a ContextVec with the added ability to access elements via String keys.
///
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
/// *   key values are not reused
/// *   keys.len() == vector.len()

impl<T> KeyedContextVec<T> {
    /// Create a new empty KeyedContextVec
    pub fn new() -> KeyedContextVec<T> {
        KeyedContextVec {
            context: vec!(),
            vector:  vec!(),
            keys:    vec!(),
        }
    }

    /// Create a new KeyedContextVec from a Vec
    pub fn from_vec(pairs: Vec<(String, T)>) -> KeyedContextVec<T> {
        let mut vector: Vec<T> = vec!();
        let mut keys: Vec<String> = vec!();

        for (key, item) in pairs {
            keys.push(key);
            vector.push(item);
        }

        KeyedContextVec {
            context: vec!(),
            vector:  vector,
            keys:    keys,
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

    /// Consume the KeyedContextVec into the context
    pub fn into_context(self) -> Vec<usize> {
        self.context
    }

    /// Consume the KeyedContextVec into the vector
    pub fn into_vector(self) -> Vec<T> {
        self.vector
    }

    /// Consume the KeyedContextVec into a tuple of the context, keys and the vector
    pub fn into_tuple(self) -> (Vec<usize>, Vec<String>, Vec<T>) {
        (self.context, self.keys, self.vector)
    }

    /// Clears the context
    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    /// Sets a new context
    pub fn set_context(&mut self, value: usize) {
        let length = self.vector.len();
        if value >= length {
            panic!(format!("Attempted to set context {} on a KeyedContextVec of length {}", value, length));
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
                panic!(format!("Attempted to set context {} on a KeyedContextVec of length {}", value, length));
            }
            self.context.push(value);
        }
    }

    /// Set to a new vector.
    /// Clears the context.
    pub fn set_vec(&mut self, pairs: Vec<(String, T)>) {
        self.context.clear();
        self.vector = vec!();
        self.keys = vec!();

        for (key, value) in pairs {
            self.vector.push(value);
            self.keys.push(key);
        }
    }

    /// Clears the keys/values and the context
    pub fn clear(&mut self) {
        self.context.clear();
        self.vector.clear();
        self.keys.clear();
    }

    /// Push a value to the end of the vector
    pub fn push(&mut self, key: String, value: T) {
        if self.keys.contains(&key) {
            panic!("Key is already used.");
        }

        self.vector.push(value);
        self.keys.push(key);
    }

    /// Insert a value into the vector.
    /// Invalid context indices are updated.
    pub fn insert(&mut self, index: usize, key: String, value: T) {
        if self.keys.contains(&key) {
            panic!("Key is already used.");
        }

        self.vector.insert(index, value);
        self.keys.insert(index, key);

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
                self.keys.pop();

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

    /// Retrieve the index corresponding to the given key
    /// This operation is O(n)
    pub fn key_to_value(&self, key_search: &str) -> Option<&T> {
        for (value, key_current) in self.vector.iter().zip(&self.keys) {
            if key_search == key_current {
                return Some(value);
            }
        }
        None
    }

    /// Retrieve the index corresponding to the given key
    /// This operation is O(n)
    pub fn key_to_value_mut(&mut self, key_search: &str) -> Option<&mut T> {
        for (value, key_current) in self.vector.iter_mut().zip(&self.keys) {
            if key_search == key_current {
                return Some(value);
            }
        }
        None
    }

    /// Retrive the key corresponding to the given index
    pub fn index_to_key(&self, i: usize) -> Option<String> {
        self.keys.get(i).map(|x| x.clone())
    }

    /// Retrieve the index corresponding to the given key
    /// This operation is O(n)
    pub fn key_to_index(&self, key_search: &str) -> Option<usize> {
        for (i, key_current) in self.keys.iter().enumerate() {
            if key_search == key_current {
                return Some(i);
            }
        }
        None
    }

    /// Iterate over key/value pairs
    pub fn key_value_iter(&self) -> Zip<Iter<String>, Iter<T>> {
        self.keys.iter().zip(&self.vector)
    }

    /// Iterate over keys
    pub fn key_iter(&self) -> Iter<String> {
        self.keys.iter()
    }

    /// Create a vector of keys
    pub fn keys(&self) -> Vec<String> {
        self.keys.clone()
    }

    /// Returns true if the passed key is used
    pub fn contains_key(&self, key: &String) -> bool {
        self.keys.contains(key)
    }

    fn format_keys(&self) -> String {
        let keys = self.keys.iter().map(|x| format!("'{}'", x));
        join(keys, ", ")
    }
}

impl<'a, T> Index<&'a str> for KeyedContextVec<T> {
    type Output = T;
    fn index(&self, key: &'a str) -> &T {
        self.key_to_value(key).unwrap()
    }
}

impl<'a, T> IndexMut<&'a str> for KeyedContextVec<T> {
    fn index_mut(&mut self, key: &'a str) -> &mut T {
        self.key_to_value_mut(key).unwrap()
    }
}

// The following is copied from https://doc.rust-lang.org/src/collections/vec.rs.html

impl<T> Index<usize> for KeyedContextVec<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &T {
        // NB built-in indexing via `&[T]`
        &(**self)[index]
    }
}

impl<T> IndexMut<usize> for KeyedContextVec<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut T {
        // NB built-in indexing via `&mut [T]`
        &mut (**self)[index]
    }
}

impl<T> Index<Range<usize>> for KeyedContextVec<T> {
    type Output = [T];

    #[inline]
    fn index(&self, index: Range<usize>) -> &[T] {
        Index::index(&**self, index)
    }
}

impl<T> Index<RangeTo<usize>> for KeyedContextVec<T> {
    type Output = [T];

    #[inline]
    fn index(&self, index: RangeTo<usize>) -> &[T] {
        Index::index(&**self, index)
    }
}

impl<T> Index<RangeFrom<usize>> for KeyedContextVec<T> {
    type Output = [T];

    #[inline]
    fn index(&self, index: RangeFrom<usize>) -> &[T] {
        Index::index(&**self, index)
    }
}

impl<T> Index<RangeFull> for KeyedContextVec<T> {
    type Output = [T];

    #[inline]
    fn index(&self, _index: RangeFull) -> &[T] {
        self
    }
}

impl<T> IndexMut<Range<usize>> for KeyedContextVec<T> {
    #[inline]
    fn index_mut(&mut self, index: Range<usize>) -> &mut [T] {
        IndexMut::index_mut(&mut **self, index)
    }
}

impl<T> IndexMut<RangeTo<usize>> for KeyedContextVec<T> {
    #[inline]
    fn index_mut(&mut self, index: RangeTo<usize>) -> &mut [T] {
        IndexMut::index_mut(&mut **self, index)
    }
}

impl<T> IndexMut<RangeFrom<usize>> for KeyedContextVec<T> {
    #[inline]
    fn index_mut(&mut self, index: RangeFrom<usize>) -> &mut [T] {
        IndexMut::index_mut(&mut **self, index)
    }
}

impl<T> IndexMut<RangeFull> for KeyedContextVec<T> {
    #[inline]
    fn index_mut(&mut self, _index: RangeFull) -> &mut [T] {
        self
    }
}

// End copied section

impl<T> Deref for KeyedContextVec<T> {
    type Target = [T];

	#[inline]
    fn deref(&self) -> &[T] {
        &self.vector
    }
}

impl<T> DerefMut for KeyedContextVec<T> {
	#[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        &mut self.vector
    }
}

impl<T> Node for KeyedContextVec<T> where T: Node + Serialize + DeserializeOwned + Default {
    fn node_step(&mut self, mut runner: NodeRunner) -> String {
        match runner.step() {
            NodeToken::ChainIndex (index) => {
                let length = self.vector.len();
                match self.vector.get_mut(index) {
                    Some (item) => item.node_step(runner),
                    None => {
                        return match length {
                             0 => format!("Used index {} on an empty keyed context vector", index),
                             1 => format!("Used index {} on a keyed context vector of size 1 (try 0)", index),
                             _ => format!("Used index {} on a keyed context vector of size {} (try a value between 0-{})", index, length, length-1)
                        }
                    }
                }
            }
            NodeToken::ChainKey (key) => {
                let length = self.vector.len();
                match self.key_to_value_mut(&key) {
                    Some (item) => { return item.node_step(runner) }
                    None => { }
                }
                match length {
                     0 => {
                        format!("Used key '{}' on an empty keyed context vector.", key)
                     }
                     _ => {
                        format!("Used key '{}' on a keyed context vector that does not contain it. Try one of: {}", key, self.format_keys())
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
            NodeToken::ChainProperty (ref s) if s == "length" => { self.vector.len().node_step(runner) }
            NodeToken::Get => {
                serde_json::to_string_pretty(&mut self.vector).unwrap()
            }
            NodeToken::Set (value) => {
                match serde_json::from_str(&value) {
                    Ok(result) => {
                        self.vector = result;
                        String::from("")
                    }
                    Err(err) => {
                        format!("keyed context vector set error: {}", err)
                    }
                }
            }
            NodeToken::InsertKey (key) => {
                if self.contains_key(&key) {
                    format!("Tried to insert with key '{}' on a keyed context vector that already contains it. Current keys: {}", key, self.format_keys())
                } else {
                    self.push(key, T::default());
                    String::new()
                }
            }
            NodeToken::InsertIndexKey (index, key) => {
                let max_index = self.len();
                if index > max_index {
                    format!("Tried to insert at index {} on a keyed context vector of size {} (try a value between 0-{})", index, max_index, max_index)
                } else if self.contains_key(&key) {
                    format!("Tried to insert with key '{}' on a keyed context vector that already contains it. Current keys: {}", key, self.format_keys())
                } else {
                    self.insert(index, key, T::default());
                    String::new()
                }
            }
            NodeToken::Remove => {
                if self.len() == 0 {
                    String::from("Tried to remove from an empty keyed context vector.")
                } else {
                    self.pop();
                    String::new()
                }
            }
            NodeToken::RemoveIndex (index) => {
                let max_index = self.len() - 1;
                if index > max_index {
                    format!("Tried to remove the value at index {} on a keyed context vector of size {} (try a value between 0-{})", index, self.len(), max_index)
                }
                else {
                    self.remove(index);
                    String::new()
                }
            }
            NodeToken::RemoveKey (key) => {
                if let Some(index) = self.key_to_index(&key) {
                    self.remove(index);
                    String::new()
                } else {
                    format!("Tried to remove the value with key '{}' on a keyed context vector that doesnt contain it. Current keys: {}", key, self.format_keys())
                }
            }
            NodeToken::SetDefault => {
                self.vector = vec!();
                String::new()
            }
            NodeToken::Help => {
                String::from(r#"
Keyed Context Vector Help

Commands:
*   help               - display this help
*   get                - display JSON
*   set                - set to JSON
*   insert $KEY        - create a new element at the end of the vector with $KEY
*   insert $INDEX $KEY - create a new element at $INDEX with $KEY
*   remove             - remove the element
*   remove $KEY        - remove the element with $KEY
*   remove $INDEX      - remove the element at $INDEX
*   default            - reset to default values

Accessors:
*   [INDEX] - access item at INDEX
*   [?]     - access items at current context
*   .length - display number of items"#)
            }
            action => { format!("keyed context vector cannot '{:?}'", action) }
        }
    }
}
