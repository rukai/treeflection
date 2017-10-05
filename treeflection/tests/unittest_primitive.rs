extern crate treeflection;

use treeflection::{Node, NodeRunner, NodeToken};
use std::fmt::Debug;

fn assert_set<T>(mut node: T, set: &str, expected: T) where T: Node + Debug + PartialEq {
    let tokens = vec!(NodeToken::Set(String::from(set)));
    assert_eq!(node.node_step(NodeRunner { tokens: tokens }), String::new());
    assert_eq!(expected, node);
}

fn assert_get<T: Node>(mut node: T, expected: &str) {
    let tokens = vec!(NodeToken::Get);
    let result = node.node_step(NodeRunner { tokens: tokens });
    assert_eq!(expected, String::from(result));
}

#[test]
fn string_set() {
    assert_set::<String>(String::from("Foobar"), "string set", String::from("string set"));
}

#[test]
fn string_get() {
    assert_get::<String>(String::from("foobar"), "foobar");
}

#[test]
fn string_copy_paste() {
    let copy_token = NodeRunner { tokens: vec!(NodeToken::CopyFrom) };
    let paste_token = NodeRunner { tokens: vec!(NodeToken::PasteTo) };

    let mut a = String::from("copied value");
    let mut b = String::new();

    assert_eq!(a.node_step(copy_token), "");
    assert_eq!(a, String::from("copied value"));
    assert_eq!(b.node_step(paste_token), "");
    assert_eq!(b, String::from("copied value"));
}

#[test]
fn string_help() {
    let output = r#"
String Help

Valid values: Anything

Commands:
*   help  - display this help
*   copy  - copy this value
*   paste - paste the copied value here
*   get   - display value
*   set   - set to value"#;
    let mut value = String::from("YO");
    let runner = NodeRunner { tokens: vec!( NodeToken::Help ) };
    assert_eq!(value.node_step(runner).as_str(), output);
}

#[test]
fn bool_set() {
    assert_set::<bool>(true, "false", false);
    assert_set::<bool>(false, "true", true);
    assert_set::<bool>(true, "true", true);
    assert_set::<bool>(false, "false", false);
}

#[test]
fn bool_get() {
    assert_get::<bool>(true, "true");
    assert_get::<bool>(false, "false");
}

#[test]
fn bool_help() {
    let output = r#"
Bool Help

Valid values: true or false

Commands:
*   help - display this help
*   get  - display value
*   set  - set to value"#;
    let mut value = true;
    let runner = NodeRunner { tokens: vec!( NodeToken::Help ) };
    assert_eq!(value.node_step(runner).as_str(), output);
}
