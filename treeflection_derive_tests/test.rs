// TODO: These tests are in this crate as a workaround for testing treeflection_derive
// while tests folder does not work with macros 1.1 https://github.com/rust-lang/rust/issues/37480
// alternatively we could move the other tests into this crate

#![feature(proc_macro)]

extern crate treeflection;
#[macro_use] extern crate treeflection_derive;

use treeflection::{Node, NodeRunner, NodeToken};

#[derive(Node)]
struct Parent {
    pub foo: String,
    pub bar: u32,
    pub baz: bool,
    pub child: Child,
    private: i64,
}

#[derive(Node)]
struct Child {
    pub qux: i32,
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
            private: 1337,
        }
    }
}

#[test]
fn get() {
    let runner = NodeRunner { tokens: vec!(NodeToken::Get) };
    assert_eq!(Parent::new().node_step(runner), String::from("This is a struct"));
}

#[test]
fn copy() {
    let runner = NodeRunner { tokens: vec!(NodeToken::CopyFrom) };
    assert_eq!(Parent::new().node_step(runner), String::from("Parent cannot 'CopyFrom'"));
}

#[test]
fn no_property() {
    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainProperty(String::from("notfoo")),
    )};
    assert_eq!(Parent::new().node_step(runner), String::from("Parent does not have a property 'notfoo'"));
}

#[test]
fn private_property() {
    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainProperty(String::from("private")),
    )};
    assert_eq!(Parent::new().node_step(runner), String::from("Parent does not have a property 'private'"));
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
