extern crate treeflection;

use treeflection::{Node, NodeRunner, NodeToken};

fn test_vec() -> Vec<i32> {
    vec!(100000, 13, -358, 42)
}

#[test]
fn vec_chain_property() {
    let runner = NodeRunner { tokens:  vec!(
        NodeToken::Get,
        NodeToken::ChainIndex(0),
    )};
    assert_eq!("100000", test_vec().node_step(runner));

    let runner = NodeRunner { tokens:  vec!(
        NodeToken::Get,
        NodeToken::ChainIndex(1),
    )};
    assert_eq!("13", test_vec().node_step(runner));

    let runner = NodeRunner { tokens:  vec!(
        NodeToken::Get,
        NodeToken::ChainIndex(2),
    )};
    assert_eq!("-358", test_vec().node_step(runner));

    let runner = NodeRunner { tokens:  vec!(
        NodeToken::Get,
        NodeToken::ChainIndex(3),
    )};
    assert_eq!("42", test_vec().node_step(runner));

    let runner = NodeRunner { tokens:  vec!(
        NodeToken::Get,
        NodeToken::ChainIndex(4),
    )};
    assert_eq!(test_vec().node_step(runner), "Used index 4 on a vector of size 4 (try a value between 0-3)");
}

#[test]
fn vec_get() {
    let runner = NodeRunner { tokens: vec!(NodeToken::Get) };
    assert_eq!("[100000,13,-358,42]", test_vec().node_step(runner));
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
