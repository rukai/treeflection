use serde::{Serialize, Deserialize};
use serde_json;

use ::node_runner::NodeRunner;
use ::node_token::NodeToken;

pub trait Node {
    fn node_step(&mut self, runner: NodeRunner) -> String;
}

impl<T> Node for Vec<T> where T: Node + Serialize + Deserialize + Default {
    fn node_step(&mut self, mut runner: NodeRunner) -> String {
        match runner.step() {
            NodeToken::ChainIndex (index) => {
                let length = self.len();
                match self.get_mut(index) {
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
            NodeToken::ChainProperty (ref s) if s == "length" => { self.len().node_step(runner) } // TODO: yeah this should really be a command not a property
            NodeToken::Get => {
                serde_json::to_string_pretty(self).unwrap()
            }
            NodeToken::Set (value) => {
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
            NodeToken::Insert (index) => {
                let max_index = self.len();
                if index > max_index {
                    format!("Tried to insert at index {} on a vector of size {} (try a value between 0-{})", index, max_index, max_index)
                }
                else {
                    self.insert(index, T::default());
                    String::new()
                }
            }
            NodeToken::Remove (index) => {
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
                *self = vec!();
                String::new()
            }
            NodeToken::Help => {
                String::from(r#"
Vector Help

Commands:
*   help    - display this help
*   get     - display JSON
*   set     - set to JSON
*   insert  - create a new element
*   remove  - remove an element
*   default - reset to default values

Accessors:
*   [index] - access item at index
*   .length - display number of items"#)
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
                    NodeToken::Help => {
                        String::from(r#"
Tuple Help

Commands:
*   help - display this help
*   get  - display JSON
*   set  - set to JSON

Accessors:
*   [index] - access item at index"#)
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
            NodeToken::Help        => {
                String::from(r#"
Bool Help

Valid values: true or false

Commands:
*   help - display this help
*   get  - display value
*   set  - set to value"#)
            }
            action                 => { format!("bool cannot '{:?}'", action) }
        }
    }
}

impl Node for String {
    fn node_step(&mut self, mut runner: NodeRunner) -> String {
        match runner.step() {
            NodeToken::Get         => { (*self).clone() }
            NodeToken::Set (value) => { *self = value; String::from("") }
            NodeToken::Help        => {
                String::from(r#"
String Help

Valid values: Anything

Commands:
*   help - display this help
*   get  - display value
*   set  - set to value"#)
            }
            action                 => { format!("String cannot '{:?}'", action) }
        }
    }
}

macro_rules! int_node {
    ($e:ty, $valid_values:tt) => {
        impl Node for $e {
            fn node_step(&mut self, mut runner: NodeRunner) -> String {
                match runner.step() {
                    NodeToken::Get         => { (*self).to_string() }
                    NodeToken::Set (value) => {
                        match value.parse() {
                            Ok (value) => {
                                *self = value;
                                String::from("")
                            }
                            Err(_) => {
                                format!("Invalid value for {} (needs to be: {})", stringify! { $e }, $valid_values)
                            }
                        }
                    }
                    NodeToken::Help        => {
                        format!(r#"
{} Help

Valid values: {}

Commands:
*   help - display this help
*   get  - display value
*   set  - set to value"#,
                            stringify! { $e },
                            $valid_values
                        )
                    }
                    action                 => { format!("{} cannot '{:?}'", stringify! { $e }, action) }
                }
            }
        }
    }
}

int_node!(i64, "A number from –9,223,372,036,854,775,808 to 9,223,372,036,854,775,807");
int_node!(u64, "A number from 0 to 18,446,744,073,709,551,615");
int_node!(i32, "A number from –2,147,483,648 to 2,147,483,647");
int_node!(u32, "A number from 0 to 4,294,967,295");
int_node!(i16, "A number from –32,768 to –32,767");
int_node!(u16, "A number from 0 to 65,535");
int_node!(i8, "A number from -128 to 127");
int_node!(u8, "A number from 0 to 255");
int_node!(isize, "A number from –9,223,372,036,854,775,808 to 9,223,372,036,854,775,807");
int_node!(usize, "A number from 0 to 18,446,744,073,709,551,615");

// TODO: Not sure how to best present possible values for the floats
int_node!(f32, "A number with a decimal point");
int_node!(f64, "A higher precision number with a decimal point");
