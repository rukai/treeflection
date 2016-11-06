extern crate treeflection;

use treeflection::{Node, NodeRunner, NodeToken};
use std::fmt::Debug;

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
