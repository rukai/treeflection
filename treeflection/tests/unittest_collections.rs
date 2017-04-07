extern crate treeflection;

use treeflection::{Node, NodeRunner, NodeToken};

fn test_vec4() -> Vec<i32> {
    vec!(100000, 13, -358, 42)
}

fn test_vec1() -> Vec<i32> {
    vec!(13)
}

fn test_vec0() -> Vec<i32> {
    vec!()
}

fn test_tuple() -> (i32, bool) {
    (42, true)
}

fn test_tuple16() -> (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8) {
    (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15)
}

#[test]
fn vec_chain_index() {
    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainIndex(0),
    )};
    assert_eq!("100000", test_vec4().node_step(runner));

    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainIndex(1),
    )};
    assert_eq!("13", test_vec4().node_step(runner));

    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainIndex(2),
    )};
    assert_eq!("-358", test_vec4().node_step(runner));

    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainIndex(3),
    )};
    assert_eq!("42", test_vec4().node_step(runner));

    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainIndex(4),
    )};
    assert_eq!(test_vec4().node_step(runner), "Used index 4 on a vector of size 4 (try a value between 0-3)");

    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainIndex(1),
    )};
    assert_eq!(test_vec1().node_step(runner), "Used index 1 on a vector of size 1 (try 0)");

    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainIndex(0),
    )};
    assert_eq!(test_vec0().node_step(runner), "Used index 0 on an empty vector");
}

#[test]
fn vec_insert() {
    let mut some_vec = test_vec4();

    assert_eq!(some_vec.len(), 4);
    assert_eq!(some_vec[0], 100000);
    assert_eq!(some_vec[1], 13);
    assert_eq!(some_vec[2], -358);
    assert_eq!(some_vec[3], 42);

    let runner = NodeRunner { tokens: vec!(NodeToken::Insert(5)) };
    assert_eq!(some_vec.node_step(runner), "Tried to insert at index 5 on a vector of size 4 (try a value between 0-4)");
    assert_eq!(some_vec.len(), 4);
    assert_eq!(some_vec[0], 100000);
    assert_eq!(some_vec[1], 13);
    assert_eq!(some_vec[2], -358);
    assert_eq!(some_vec[3], 42);

    let runner = NodeRunner { tokens: vec!(NodeToken::Insert(0)) };
    assert_eq!(some_vec.node_step(runner), "");
    assert_eq!(some_vec.len(), 5);
    assert_eq!(some_vec[0], 0);
    assert_eq!(some_vec[1], 100000);
    assert_eq!(some_vec[2], 13);
    assert_eq!(some_vec[3], -358);
    assert_eq!(some_vec[4], 42);

    let runner = NodeRunner { tokens: vec!(NodeToken::Insert(2)) };
    assert_eq!(some_vec.node_step(runner), "");
    assert_eq!(some_vec.len(), 6);
    assert_eq!(some_vec[0], 0);
    assert_eq!(some_vec[1], 100000);
    assert_eq!(some_vec[2], 0);
    assert_eq!(some_vec[3], 13);
    assert_eq!(some_vec[4], -358);
    assert_eq!(some_vec[5], 42);
}

#[test]
fn vec_remove() {
    let mut some_vec = test_vec4();

    assert_eq!(some_vec.len(), 4);
    assert_eq!(some_vec[0], 100000);
    assert_eq!(some_vec[1], 13);
    assert_eq!(some_vec[2], -358);
    assert_eq!(some_vec[3], 42);

    let runner = NodeRunner { tokens: vec!(NodeToken::Remove(4)) };
    assert_eq!(some_vec.node_step(runner), "Tried to remove the value at index 4 on a vector of size 4 (try a value between 0-3)");
    assert_eq!(some_vec.len(), 4);
    assert_eq!(some_vec[0], 100000);
    assert_eq!(some_vec[1], 13);
    assert_eq!(some_vec[2], -358);
    assert_eq!(some_vec[3], 42);

    let runner = NodeRunner { tokens: vec!(NodeToken::Remove(0)) };
    assert_eq!("", some_vec.node_step(runner));
    assert_eq!(some_vec.len(), 3);
    assert_eq!(some_vec[0], 13);
    assert_eq!(some_vec[1], -358);
    assert_eq!(some_vec[2], 42);

    let runner = NodeRunner { tokens: vec!(NodeToken::Remove(2)) };
    assert_eq!("", some_vec.node_step(runner));
    assert_eq!(some_vec.len(), 2);
    assert_eq!(some_vec[0], 13);
    assert_eq!(some_vec[1], -358);
}

#[test]
fn vec_reset() {
    let mut some_vec = test_vec4();
    let runner = NodeRunner { tokens: vec!(NodeToken::SetDefault) };

    assert_eq!(4, some_vec.len());
    some_vec.node_step(runner);
    assert_eq!(0, some_vec.len());
}

#[test]
fn vec_get() {
    let runner = NodeRunner { tokens: vec!(NodeToken::Get) };
    assert_eq!("[\n  100000,\n  13,\n  -358,\n  42\n]", test_vec4().node_step(runner));
}

#[test]
fn vec_set() {
    let mut some_vec = test_vec4();
    let runner = NodeRunner { tokens: vec!(NodeToken::Set(String::from("[1, 2, 99, 100]"))) };
    assert_eq!(some_vec.node_step(runner), String::from(""));
    assert_eq!(1, some_vec[0]);
    assert_eq!(2, some_vec[1]);
    assert_eq!(99, some_vec[2]);
    assert_eq!(100, some_vec[3]);
}

#[test]
fn vec_set_fail() {
    let mut some_vec = test_vec4();
    let runner = NodeRunner { tokens: vec!(NodeToken::Set(String::from("[1, lol]"))) };
    assert_eq!(some_vec.node_step(runner), String::from("vector set error: expected value at line 1 column 5"));
}

#[test]
fn vec_help() {
    let output = r#"
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
*   .length - display number of items"#;

    let mut some_vec = test_vec4();
    let runner = NodeRunner { tokens: vec!(NodeToken::Help) };
    assert_eq!(some_vec.node_step(runner), String::from(output));
}

#[test]
fn tuple_chain_index() {
    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainIndex(0),
    )};
    assert_eq!("42", test_tuple().node_step(runner));

    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainIndex(1),
    )};
    assert_eq!("true", test_tuple().node_step(runner));

    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainIndex(2),
    )};
    assert_eq!(test_tuple().node_step(runner), "Used index 2 on a ( T0 , T1 , )");
}

#[test]
fn tuple_get() {
    let runner = NodeRunner { tokens: vec!(NodeToken::Get) };
    assert_eq!(test_tuple().node_step(runner), String::from("[\n  42,\n  true\n]"));

    let runner = NodeRunner { tokens: vec!(NodeToken::Get) };
    assert_eq!(test_tuple16().node_step(runner), String::from("[\n  0,\n  1,\n  2,\n  3,\n  4,\n  5,\n  6,\n  7,\n  8,\n  9,\n  10,\n  11,\n  12,\n  13,\n  14,\n  15\n]"));
}

#[test]
fn tuple_set() {
    let mut some_tuple = test_tuple();
    let runner = NodeRunner { tokens: vec!(NodeToken::Set(String::from("[1337,false]"))) };
    assert_eq!(some_tuple.node_step(runner), String::from(""));
    assert_eq!(some_tuple.0, 1337);
    assert_eq!(some_tuple.1, false);

    let mut some_tuple = test_tuple16();
    let runner = NodeRunner { tokens: vec!(NodeToken::Set(
        String::from("[100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115]")
    )) };
    assert_eq!(some_tuple.node_step(runner), String::from(""));
    assert_eq!(some_tuple.0, 100);
    assert_eq!(some_tuple.1, 101);
    assert_eq!(some_tuple.2, 102);
    assert_eq!(some_tuple.3, 103);
    assert_eq!(some_tuple.4, 104);
    assert_eq!(some_tuple.5, 105);
    assert_eq!(some_tuple.6, 106);
    assert_eq!(some_tuple.7, 107);
    assert_eq!(some_tuple.8, 108);
    assert_eq!(some_tuple.9, 109);
    assert_eq!(some_tuple.10, 110);
    assert_eq!(some_tuple.11, 111);
    assert_eq!(some_tuple.12, 112);
    assert_eq!(some_tuple.13, 113);
    assert_eq!(some_tuple.14, 114);
    assert_eq!(some_tuple.15, 115);
}

#[test]
fn tuple_set_fail() {
    let mut some_tuple = test_tuple();
    let runner = NodeRunner { tokens: vec!(NodeToken::Set(String::from("[1, 2]"))) };
    assert_eq!(some_tuple.node_step(runner), String::from("( T0 , T1 , ) set error: invalid type: integer `2`, expected a boolean at line 1 column 5"));
    assert_eq!(some_tuple.0, 42);
    assert_eq!(some_tuple.1, true);
}

#[test]
fn tuple_help() {
    let output = r#"
Tuple Help

Commands:
*   help - display this help
*   get  - display JSON
*   set  - set to JSON

Accessors:
*   [index] - access item at index"#;
    let mut some_tuple = test_tuple();
    let runner = NodeRunner { tokens: vec!(NodeToken::Help) };
    assert_eq!(some_tuple.node_step(runner), String::from(output));
}

#[test]
fn option_help() {
    let output = r#"
Option Help

Commands:
*   help    - display this help
*   get     - display JSON
*   set     - set to JSON
*   insert  - set to a value
*   remove  - remove value
*   default - remove value

Accessors:
*   .value - the stored value number of items"#;

    let mut some_option: Option<usize> = None;
    let runner = NodeRunner { tokens: vec!(NodeToken::Help) };
    assert_eq!(some_option.node_step(runner), String::from(output));
}

#[test]
fn option_get() {
    let mut some_option: Option<usize> = None;
    let runner = NodeRunner { tokens: vec!(NodeToken::Get) };
    assert_eq!("null", some_option.node_step(runner));

    some_option = Some(1337);
    let runner = NodeRunner { tokens: vec!(NodeToken::Get) };
    assert_eq!("1337", some_option.node_step(runner));
}

#[test]
fn option_set() {
    let mut some_option: Option<usize> = Some(358);
    let runner = NodeRunner { tokens: vec!(NodeToken::Set(String::from("null"))) };
    assert_eq!(some_option.node_step(runner), String::from(""));
    assert!(some_option.is_none());

    let runner = NodeRunner { tokens: vec!(NodeToken::Set(String::from("42"))) };
    assert_eq!(some_option.node_step(runner), String::from(""));
    assert_eq!(42, some_option.unwrap());
}

#[test]
fn option_set_fail() {
    let mut some_option: Option<usize> = Some(358);
    let runner = NodeRunner { tokens: vec!(NodeToken::Set(String::from("None"))) };
    assert_eq!(some_option.node_step(runner), String::from("Option set error: expected value at line 1 column 1"));
    if let Some(value) = some_option {
        assert_eq!(358, value);
    }
    else {
        assert!(false);
    }
}

#[test]
fn option_insert() {
    let mut some_option: Option<usize> = None;
    let runner = NodeRunner { tokens: vec!(NodeToken::Insert(9)) };
    assert_eq!(some_option.node_step(runner), String::from(""));
    if let Some(value) = some_option {
        assert_eq!(0, value);
    }
    else {
        assert!(false);
    }
}

#[test]
fn option_remove() {
    let mut some_option: Option<usize> = Some(358);
    let runner = NodeRunner { tokens: vec!(NodeToken::Remove(42)) };
    assert_eq!(some_option.node_step(runner), String::from(""));
    assert!(some_option.is_none());
}

#[test]
fn option_default() {
    let mut some_option: Option<usize> = Some(358);
    let runner = NodeRunner { tokens: vec!(NodeToken::SetDefault) };
    assert_eq!(some_option.node_step(runner), String::from(""));
    assert!(some_option.is_none());
}

#[test]
fn option_value() {
    let mut some_option: Option<usize> = Some(42);
    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainProperty(String::from("value")),
    )};
    assert_eq!("42", some_option.node_step(runner));

    some_option = None;
    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainProperty(String::from("value")),
    )};
    assert_eq!("Option contains no value", some_option.node_step(runner));
}
