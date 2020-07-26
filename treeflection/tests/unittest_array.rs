extern crate treeflection;

use treeflection::{Node, NodeRunner, NodeToken};

fn test_array2() -> [bool; 2] {
    [false, true]
}

fn test_array16() -> [u8; 16] {
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
}

#[test]
fn array_chain_index() {
    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainIndex(0),
    )};
    assert_eq!("false", test_array2().node_step(runner));

    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainIndex(1),
    )};
    assert_eq!("true", test_array2().node_step(runner));

    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainIndex(2),
    )};
    assert_eq!(test_array2().node_step(runner), "Used index 2 on an array of length 2");
}

#[test]
fn array_chain_all() {
    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainAll,
    )};
    assert_eq!("|false|true|", test_array2().node_step(runner));
}

#[test]
fn array_get() {
    let runner = NodeRunner { tokens: vec!(NodeToken::Get) };
    assert_eq!(test_array2().node_step(runner), String::from("[\n  false,\n  true\n]"));

    let runner = NodeRunner { tokens: vec!(NodeToken::Get) };
    assert_eq!(test_array16().node_step(runner), String::from("[\n  0,\n  1,\n  2,\n  3,\n  4,\n  5,\n  6,\n  7,\n  8,\n  9,\n  10,\n  11,\n  12,\n  13,\n  14,\n  15\n]"));
}

#[test]
fn array_set() {
    let mut some_array = test_array2();
    let runner = NodeRunner { tokens: vec!(NodeToken::Set(String::from("[true,true]"))) };
    assert_eq!(some_array.node_step(runner), String::from(""));
    assert_eq!(some_array[0], true);
    assert_eq!(some_array[1], true);

    let mut some_array = test_array16();
    let runner = NodeRunner { tokens: vec!(NodeToken::Set(
        String::from("[100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115]")
    )) };
    assert_eq!(some_array.node_step(runner), String::from(""));
    assert_eq!(some_array[0], 100);
    assert_eq!(some_array[1], 101);
    assert_eq!(some_array[2], 102);
    assert_eq!(some_array[3], 103);
    assert_eq!(some_array[4], 104);
    assert_eq!(some_array[5], 105);
    assert_eq!(some_array[6], 106);
    assert_eq!(some_array[7], 107);
    assert_eq!(some_array[8], 108);
    assert_eq!(some_array[9], 109);
    assert_eq!(some_array[10], 110);
    assert_eq!(some_array[11], 111);
    assert_eq!(some_array[12], 112);
    assert_eq!(some_array[13], 113);
    assert_eq!(some_array[14], 114);
    assert_eq!(some_array[15], 115);
}

#[test]
fn array_set_fail() {
    let mut some_array = test_array2();
    let runner = NodeRunner { tokens: vec!(NodeToken::Set(String::from("[false, 2]"))) };
    assert_eq!(some_array.node_step(runner), String::from("array set error: invalid type: integer `2`, expected a boolean at line 1 column 9"));
    assert_eq!(some_array[0], false);
    assert_eq!(some_array[1], true);
}

#[test]
fn array_help() {
    let output = r#"
Array Help

Commands:
*   help - display this help
*   get  - display JSON
*   set  - set to JSON

Accessors:
*   [index] - access item at index"#;
    let mut some_array = test_array2();
    let runner = NodeRunner { tokens: vec!(NodeToken::Help) };
    assert_eq!(some_array.node_step(runner), String::from(output));
}
