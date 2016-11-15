use std::vec::Vec;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde_json;

use ::node::Node;
use ::node_runner::NodeRunner;
use ::node_token::NodeToken;

pub struct ContextVec<T> {
    context: Vec<usize>,
    vector:        Vec<T>,
}

impl<T> ContextVec<T> {
    /// Create a new empty ContextVec
    pub fn new() -> ContextVec<T> {
        ContextVec {
            context: vec!(),
            vector: Vec::<T>::new(),
        }
    }

    /// Create a new ContextVec from a Vec
    pub fn from_vec(vector: Vec<T>) -> ContextVec<T> {
        ContextVec {
            context: vec!(),
            vector: vector,
        }
    }

    /// Get a reference to the context
    pub fn get_context(&self) -> &Vec<usize> {
        &self.context
    }

    /// Get a reference to the vector
    pub fn get_vec(&self) -> &Vec<T> {
        &self.vector
    }

    /// Clears the context
    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    // TODO: Could speed up set_context* by only running checks in dev builds
    /// Clears the old context, sets to a new single value context
    pub fn set_context(&mut self, value: usize) {
        let length = self.vector.len();
        if value >= length {
            panic!(format!("Attempted to set context {} on a ContextVec of length {}", value, length));
        }
        self.context.clear();
        self.context.push(value);
    }

    /// Clears the old context, sets to a new context
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

    /// Set to a new vector, clears the context
    pub fn set_vec(&mut self, vector: Vec<T>) {
        self.context.clear();
        self.vector = vector;
    }

    /// Push a value to the end of the vector
    pub fn push(&mut self, value: T) {
        self.vector.push(value);
    }

    /// Insert a value into the vector, invalid context indexes are updated
    pub fn insert(&mut self, index: usize, value: T) {
        self.vector.insert(index, value);

        for i in self.context.iter_mut() {
            if *i >= index {
                *i += 1;
            }
        }
    }

    /// Pop a value from the end of the vector, if it succeeds invalid context indexes are removed
    pub fn pop(&mut self) -> Option<T> {
        match self.vector.pop() {
            Some(value) => {
                // update context
                let len = self.vector.len();
                let mut new_context: Vec<usize> = vec!();
                for i in self.context.drain(..) {
                    if i < len {
                        new_context.push(i);
                    }
                }
                self.context = new_context;

                Some(value)
            }
            None => {
                None
            }
        }
    }
}

impl<T> Node for ContextVec<T> where T: Node + Serialize + Deserialize {
    fn node_step(&mut self, mut runner: NodeRunner) -> String {
        match runner.step() {
            NodeToken::ChainIndex (index) => {
                let length = self.vector.len();
                match self.vector.get_mut(index) {
                    Some (item) => item.node_step(runner),
                    None        => return format!("Used index {} on a vector of size {} (try a value between 0-{})", index, length, length-1)
                }
            }
            NodeToken::ChainProperty (ref s) if s == "length" => { self.vector.len().node_step(runner) }
            NodeToken::Get => {
                serde_json::to_string(&mut self.vector).unwrap()
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
            NodeToken::ChainContext => {
                let mut combined = String::from("|");
                println!("RFOLL");
                for i in self.context.iter() {
                    println!("LOL{}", i);
                    match self.vector.get_mut(*i) {
                        Some(ref mut node) => {
                            let result = node.node_step(runner.clone());
                            combined.push_str(result.as_str());
                        }
                        None => {
                            combined.push_str("Context out of range");
                        }
                    }
                    combined.push('|');
                }
                combined
            }
            action => { format!("vector cannot '{:?}'", action) }
        }
    }
}

impl<T> Serialize for ContextVec<T> where T: Serialize {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: Serializer {
        self.vector.serialize(serializer)
    }
}

impl<T> Deserialize for ContextVec<T> where T: Deserialize {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: Deserializer {
        Ok(ContextVec {
            context: vec!(),
            vector: Vec::<T>::deserialize(deserializer)?,
        })
    }
}
