extern crate treeflection;

use treeflection::{Node, NodeRunner, NodeToken};

fn test_vec() -> Vec<i32> {
    vec!(100000, 13, -358, 42)
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
    assert_eq!("100000", test_vec().node_step(runner));

    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainIndex(1),
    )};
    assert_eq!("13", test_vec().node_step(runner));

    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainIndex(2),
    )};
    assert_eq!("-358", test_vec().node_step(runner));

    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainIndex(3),
    )};
    assert_eq!("42", test_vec().node_step(runner));

    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainIndex(4),
    )};
    assert_eq!(test_vec().node_step(runner), "Used index 4 on a vector of size 4 (try a value between 0-3)");
}

#[test]
fn vec_get() {
    let runner = NodeRunner { tokens: vec!(NodeToken::Get) };
    assert_eq!("[\n  100000,\n  13,\n  -358,\n  42\n]", test_vec().node_step(runner));
}

#[test]
fn vec_set() {
    let mut some_vec = test_vec();
    let runner = NodeRunner { tokens: vec!(NodeToken::Set(String::from("[1, 2, 99, 100]"))) };
    assert_eq!(some_vec.node_step(runner), String::from(""));
    assert_eq!(1, some_vec[0]);
    assert_eq!(2, some_vec[1]);
    assert_eq!(99, some_vec[2]);
    assert_eq!(100, some_vec[3]);
}

#[test]
fn vec_set_fail()
{
    let mut some_vec = test_vec();
    let runner = NodeRunner { tokens: vec!(NodeToken::Set(String::from("[1, lol]"))) };
    assert_eq!(some_vec.node_step(runner), String::from("vector set error: expected value at line 1 column 5"));
}

#[test]
fn vec_help()
{
    let output = r#"
Vector Help

Commands:
*   help - display this help
*   get  - display JSON
*   set  - set to JSON

Accessors:
*   [index] - access item at index
*   .length - display number of items"#;

    let mut some_vec = test_vec();
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
    assert_eq!(some_tuple.node_step(runner), String::from("( T0 , T1 , ) set error: invalid type: u64 at line 1 column 5"));
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
