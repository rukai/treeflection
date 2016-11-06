// TODO: These tests are in this crate as a workaround for testing treeflection_derive
// while tests folder does not work with macros 1.1 https://github.com/rust-lang/rust/issues/37480
// alternatively we could move the other tests into this crate

#![feature(proc_macro)]

extern crate treeflection;
#[macro_use] extern crate treeflection_derive;

use treeflection::{Node, NodeRunner, NodeToken};

#[derive(Node)]
struct Parent {
    foo: String,
    bar: u32,
    baz: bool,
    child: Child,
}

impl Parent {
    fn new() -> Parent {
        Parent {
            foo: String::from("hiya"),
            bar: 42,
            baz: true,
            child: Child {
                qux: -13,
            },
        }
    }
}

#[derive(Node)]
struct Child {
    qux: i32,
}

#[test]
fn get() {
    let runner = NodeRunner { tokens: vec!(NodeToken::Get) };
    assert_eq!(Parent::new().node_step(runner), String::from("This is a struct"));
}

#[test]
fn no_property() {
    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainProperty(String::from("notfoo")),
    )};
    assert_eq!(Parent::new().node_step(runner), String::from("Package does not have a property 'notfoo'"));
}

#[test]
fn string_property() {
    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainProperty(String::from("foo")),
    )};
    assert_eq!(Parent::new().node_step(runner), String::from("hiya"));
}

#[test]
fn uint_property() {
    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainProperty(String::from("bar")),
    )};
    assert_eq!(Parent::new().node_step(runner), String::from("42"));
}

#[test]
fn bool_property() {
    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainProperty(String::from("baz")),
    )};
    assert_eq!(Parent::new().node_step(runner), String::from("true"));
}

#[test]
fn int_child_property() {
    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainProperty(String::from("qux")),
        NodeToken::ChainProperty(String::from("child")),
    )};
    assert_eq!(Parent::new().node_step(runner), "-13");
}
