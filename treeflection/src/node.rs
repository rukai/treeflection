use serde::{Serialize, Deserialize};
use serde_json;

use ::node_runner::NodeRunner;
use ::node_token::NodeToken;

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
                serde_json::to_string_pretty(self).unwrap()
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
                        serde_json::to_string_pretty(self).unwrap()
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
