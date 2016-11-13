extern crate serde;
extern crate serde_json;

use serde::{Serialize, Deserialize};

pub trait Node {
    fn node_step(&mut self, runner: NodeRunner) -> String;
}

impl<T> Node for Vec<T> where T: Node + Serialize + Deserialize {
    fn node_step(&mut self, mut runner: NodeRunner) -> String {
        match runner.step() {
            NodeToken::ChainIndex (index) => {
                let length = self.len();
                match self.get_mut(index) {
                    Some (item) => item.node_step(runner),
                    None      => return format!("Used index {} on a vector of size {} (try a value between 0-{})", index, length, length-1)
                }
            }
            NodeToken::ChainProperty (ref s) if s == "length" => { self.len().node_step(runner) }
            NodeToken::Get => {
                serde_json::to_string(self).unwrap()
            }
            NodeToken::Set(value) => {
                match serde_json::from_str(&value) {
                    Ok(result) => {
                        *self = result;
                        String::from("")
                    }
                    Err(err) => {
                        format!("vector set error: {}", err)
                    }
                }
            }
            action => { format!("vector cannot '{:?}'", action) }
        }
    }
}

macro_rules! tuple_node {
    ( $( $indexes:tt $types:ident ),* ) => {
        impl <$( $types ),*> Node for ($( $types, )*) where $( $types: Node + Serialize + Deserialize),* {
            fn node_step(&mut self, mut runner: NodeRunner) -> String {
                let name = stringify!{ ($( $types, )*) };
                match runner.step() {
                    NodeToken::ChainIndex (index) => {
                        match index {
                            $(
                                $indexes => self.$indexes.node_step(runner),
                            )*
                            _ => format!("Used index {} on a {}", index, name)
                        }
                    }
                    NodeToken::Get => {
                        serde_json::to_string(self).unwrap()
                    }
                    NodeToken::Set (value) => {
                        match serde_json::from_str(&value) {
                            Ok (result) => {
                                *self = result;
                                String::from("")
                            }
                            Err (err) => {
                                format!("{} set error: {}", name, err)
                            }
                        }
                    }
                    action => { format!("{} cannot '{:?}'", name, action) }
                }
            }
        }
    }
}

tuple_node!(0 T0);
tuple_node!(0 T0, 1 T1);
tuple_node!(0 T0, 1 T1, 2 T2);
tuple_node!(0 T0, 1 T1, 2 T2, 3 T3);
tuple_node!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4);
tuple_node!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5);
tuple_node!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6);
tuple_node!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7);
tuple_node!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8);
tuple_node!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9);
tuple_node!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9, 10 T10);
tuple_node!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9, 10 T10, 11 T11);
tuple_node!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9, 10 T10, 11 T11, 12 T12);
tuple_node!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9, 10 T10, 11 T11, 12 T12, 13 T13);
tuple_node!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9, 10 T10, 11 T11, 12 T12, 13 T13, 14 T14);
tuple_node!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9, 10 T10, 11 T11, 12 T12, 13 T13, 14 T14, 15 T15);

impl Node for bool {
    fn node_step(&mut self, mut runner: NodeRunner) -> String {
        match runner.step() {
            NodeToken::Get         => { if *self { String::from("true") } else { String::from("false") } }
            NodeToken::Set (value) => { *self = value.as_str() == "true"; String::from("") }
            action                 => { format!("bool cannot '{:?}'", action) }
        }
    }
}

impl Node for String {
    fn node_step(&mut self, mut runner: NodeRunner) -> String {
        match runner.step() {
            NodeToken::Get         => { (*self).clone() }
            NodeToken::Set (value) => { *self = value; String::from("") }
            action                 => { format!("String cannot '{:?}'", action) }
        }
    }
}

macro_rules! int_node {
    ($e:ty) => {
        impl Node for $e {
            fn node_step(&mut self, mut runner: NodeRunner) -> String {
                match runner.step() {
                    NodeToken::Get         => { (*self).to_string() }
                    NodeToken::Set (value) => { *self = value.parse().unwrap(); String::from("") }
                    action                 => { format!("{} cannot '{:?}'", stringify! { $e }, action) }
                }
            }
        }
    }
}

int_node!(i64);
int_node!(u64);
int_node!(i32);
int_node!(u32);
int_node!(i16);
int_node!(u16);
int_node!(i8);
int_node!(u8);
int_node!(isize);
int_node!(usize);
int_node!(f32);
int_node!(f64);

pub struct NodeRunner {
    pub tokens: Vec<NodeToken>
}

impl NodeRunner {
    // TODO: Currently the command must begin with a ChainProperty.
    // However there is no reason this has to be the case.
    pub fn new(command: &str) -> Result<NodeRunner, String> {
        // add first identifier to token as property
        // get next identifier, could be:
        // *   ChainProperty - starts with '.'
        // *   ChainKey      - starts with '[0-9' ends with ']'
        // *   ChainIndex    - starts with '[a-z' ends with ']'
        // repeat until space found
        // then add identifier as action including any arguments seperated by spaces
        let mut tokens: Vec<NodeToken> = vec!();
        let mut token_progress = NodeTokenProgress::ChainProperty;
        let mut token_begin = 0;

        let chars: Vec<char> = command.chars().collect();
        for (i, c_ref) in chars.iter().enumerate() {
            let c = *c_ref;
            if c == '.' || c == ' ' || c == '[' {
                tokens.push(match token_progress {
                    NodeTokenProgress::ChainProperty => {
                        let token_str = &command[token_begin..i];
                        if token_str.len() == 0 {
                            return Err (String::from("Missing property"));
                        }
                        NodeToken::ChainProperty (token_str.to_string())
                    }

                    NodeTokenProgress::ChainIndex => {
                        let token_str = &command[token_begin..i-1];
                        if token_str.len() == 0 {
                            return Err (String::from("Missing index"));
                        }
                        match command[token_begin..i-1].parse() {
                            Ok (index) => NodeToken::ChainIndex (index),
                            Err (_)    => return Err (String::from("Not a valid index"))
                        }
                    }

                    NodeTokenProgress::ChainKey => {
                        let token_str = &command[token_begin..i-1];
                        if token_str.len() == 0 {
                            return Err (String::from("Missing index"));
                        }
                        NodeToken::ChainKey (token_str.to_string())
                    }
                    NodeTokenProgress::Action => {
                        NodeToken::Get
                    }
                });
                token_begin = i+1;
            }

            match c {
                '.' => {
                    token_progress = NodeTokenProgress::ChainProperty;
                }
                ' ' => {
                    token_progress = NodeTokenProgress::Action;
                    break;
                }
                '[' => {
                    if let Some(next_c) = chars.get(i+1) {
                        if next_c.is_digit(10) {
                            token_progress = NodeTokenProgress::ChainIndex;
                        }
                        else if next_c.is_alphabetic() {
                            token_progress = NodeTokenProgress::ChainKey;
                        }
                        else {
                            return Err (String::from("Not a valid key or index."));
                        }
                    }
                    else {
                        return Err (String::from("Unfinished key or index."));
                    }
                }
                _ => { }
            }
        }

        // add action
        if let NodeTokenProgress::Action = token_progress {
            let mut action = command[token_begin..].split_whitespace();
            tokens.push(match action.next() {
                Some("get") => NodeToken::Get,
                Some("set") => {
                    match action.next() {
                        Some(arg) => NodeToken::Set(arg.to_string()),
                        None => return Err (String::from("No argument given to set action"))
                    }
                }
                Some("copy")  => NodeToken::CopyFrom,
                Some("paste") => NodeToken::PasteTo,
                Some(&_)      => return Err (String::from("Action is invalid")), // TODO: Custom actions
                None          => return Err (String::from("This should be unreachable: No Action"))
            });
        }
        else {
            return Err (String::from("No action"));
        }

        tokens.reverse();
        println!("{:?}", tokens);

        Ok(NodeRunner {
            tokens: tokens
        })
    }

    pub fn step(&mut self) -> NodeToken {
        self.tokens.pop().unwrap()
    }
}

pub enum NodeTokenProgress {
    ChainProperty,
    ChainIndex,
    ChainKey,
    Action
}

#[derive(Debug, PartialEq)]
pub enum NodeToken {
    ChainProperty (String),
    ChainIndex (usize),
    ChainKey (String),
    Get,
    Set (String),
    CopyFrom,
    PasteTo,
    Custom (String),
}
