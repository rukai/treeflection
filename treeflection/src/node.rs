use std::collections::HashMap;

use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json;

use ::node_runner::NodeRunner;
use ::node_token::NodeToken;

pub trait Node {
    fn node_step(&mut self, runner: NodeRunner) -> String;
}

impl<T> Node for Vec<T> where T: Node + Serialize + DeserializeOwned + Default {
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
            NodeToken::ChainAll => {
                let mut combined = String::from("|");
                for item in self {
                    combined.push_str(item.node_step(runner.clone()).as_ref());
                    combined.push('|');
                }
                combined
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
*   default - reset to empty vector

Accessors:
*   [index] - access item at index
*   .length - display number of items"#)
            }
            action => { format!("vector cannot '{:?}'", action) }
        }
    }
}

impl<T> Node for HashMap<String, T> where T: Node + Serialize + DeserializeOwned + Default {
    fn node_step(&mut self, mut runner: NodeRunner) -> String {
        match runner.step() {
            NodeToken::ChainKey (key) => {
                let length = self.len();
                match self.get_mut(&key) {
                    Some (item) => { return item.node_step(runner) }
                    None        => { }
                }
                match length {
                     0 => {
                        format!("Used key '{}' on an empty map.", key)
                     }
                     _ => {
                        format!("Used key '{}' on a map that does not contain it. Try one of: {}", key, format_keys(self))
                    }
                }
            }
            NodeToken::ChainAll => {
                let mut combined = String::from("|");
                let mut pairs: Vec<_> = self.iter_mut().collect();
                pairs.sort_by_key(|x| x.0);
                for (_, mut item) in pairs {
                    combined.push_str(item.node_step(runner.clone()).as_ref());
                    combined.push('|');
                }
                combined
            }
            NodeToken::GetKeys => {
                format!("Keys: {}", format_keys(self))
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
                        format!("map set error: {}", err)
                    }
                }
            }
            NodeToken::InsertKey (key) => {
                if self.contains_key(&key) {
                    format!("Tried to insert key '{}' on a map that already contains it. Current keys: {}", key, format_keys(self))
                }
                else {
                    self.insert(key, T::default());
                    String::new()
                }
            }
            NodeToken::RemoveKey (key) => {
                if let None = self.remove(&key) {
                    format!("Tried to remove key '{}' on a map that doesnt contain it. Current keys: {}", key, format_keys(self))
                }
                else {
                    String::new()
                }
            }
            NodeToken::SetDefault => {
                *self = HashMap::new();
                String::new()
            }
            NodeToken::Help => {
                String::from(r#"
Map Help

Commands:
*   help    - display this help
*   keys    - display the keys
*   get     - display JSON
*   set     - set to JSON
*   insert  - create a new element
*   remove  - remove an element
*   default - reset to empty map

Accessors:
*   [key]   - access item at the string key
*   .length - display number of items"#)
            }
            action => { format!("map cannot '{:?}'", action) }
        }
    }
}

fn format_keys<T>(map: &HashMap<String, T>) -> String {
    let mut key_list: Vec<String> = map.keys().map(|x| format!("'{}'", x)).collect();
    key_list.sort();
    key_list.join(", ")
}

macro_rules! tuple_node {
    ( $( $indexes:tt $types:ident ),* ) => {
        impl <$( $types ),*> Node for ($( $types, )*) where $( $types: Node + Serialize + DeserializeOwned),* {
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
                    NodeToken::ChainAll => {
                        let mut combined = String::from("|");
                        $(
                            combined.push_str(self.$indexes.node_step(runner.clone()).as_ref());
                            combined.push('|');
                        )*
                        combined
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
            action => { format!("bool cannot '{:?}'", action) }
        }
    }
}

impl Node for String {
    fn node_step(&mut self, mut runner: NodeRunner) -> String {
        match runner.step() {
            NodeToken::Get => { (*self).clone() }
            NodeToken::Set (value) => { *self = value; String::from("") }
            NodeToken::CopyFrom => {
                let copy = Some (self.clone());
                unsafe {
                    STRING_COPY = copy;
                }
                String::new()
            }
            NodeToken::PasteTo => {
                let paste = unsafe { STRING_COPY.clone() };
                match paste {
                    Some (value) => {
                        *self = value;
                        String::new()
                    }
                    None => {
                        String::from("String has not been copied")
                    }
                }
            }
            NodeToken::Help => {
                String::from(r#"
String Help

Valid values: Anything

Commands:
*   help  - display this help
*   copy  - copy this value
*   paste - paste the copied value here
*   get   - display value
*   set   - set to value"#)
            }
            action => { format!("String cannot '{:?}'", action) }
        }
    }
}

static mut STRING_COPY: Option<String> = None;

impl<T> Node for Option<T> where T: Node + Serialize + DeserializeOwned + Default {
    fn node_step(&mut self, mut runner: NodeRunner) -> String {
        match runner.step() {
            NodeToken::ChainProperty (ref s) if s == "value" => {
                if let &mut Some(ref mut value) = self {
                    value.node_step(runner)
                }
                else {
                    String::from("Option contains no value")
                }
            }
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
                        format!("Option set error: {}", err)
                    }
                }
            }
            NodeToken::Insert => {
                *self = Some(T::default());
                String::new()
            }
            NodeToken::Remove => {
                *self = None;
                String::new()
            }
            NodeToken::SetDefault => {
                *self = None;
                String::new()
            }
            NodeToken::Help => {
                String::from(r#"
Option Help

Commands:
*   help    - display this help
*   get     - display JSON
*   set     - set to JSON
*   insert  - set to a value
*   remove  - remove value
*   default - remove value

Accessors:
*   .value - the stored value number of items"#)
            }
            action => { format!("Option cannot '{:?}'", action) }
        }
    }
}

macro_rules! int_node {
    ($e:ty, $valid_values:tt) => {
        impl Node for $e {
            fn node_step(&mut self, mut runner: NodeRunner) -> String {
                match runner.step() {
                    NodeToken::Get => { (*self).to_string() }
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
                    NodeToken::Help => {
                        format!(r#"
{} Help

Valid values: {}

Commands:
*   help  - display this help
*   copy  - copy this value
*   paste - paste the copied value here
*   get   - display value
*   set   - set to value"#,
                            stringify! { $e },
                            $valid_values
                        )
                    }
                    NodeToken::CopyFrom => {
                        let num_copy = match stringify! { $e } {
                            "f32" | "f64" => NumStore::Float (*self as f64),
                            _             => NumStore::Int   (*self as u64)
                        };
                        unsafe {
                            NUM_COPY = num_copy;
                        }
                        String::from("")
                    }
                    NodeToken::PasteTo => {
                        let num_copy = unsafe { NUM_COPY.clone() };
                        match num_copy {
                            NumStore::Int (value) => {
                                *self = value as $e;
                                String::from("")
                            }
                            NumStore::Float (value) => {
                                *self = value as $e;
                                String::from("")
                            }
                            NumStore::None => {
                                String::from("A number has not been copied")
                            }
                        }
                    }
                    action => { format!("{} cannot '{:?}'", stringify! { $e }, action) }
                }
            }
        }
    }
}

#[derive(Clone)]
enum NumStore {
    Int   (u64),
    Float (f64),
    None,
}

static mut NUM_COPY: NumStore = NumStore::None;

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
