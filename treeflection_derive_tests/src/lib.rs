// TODO: These tests are in this crate as a workaround for testing treeflection_derive
// while tests folder does not work with macros 1.1 https://github.com/rust-lang/rust/issues/37480
// alternatively we could move the other tests into this crate

#![feature(proc_macro)]

extern crate treeflection;
#[macro_use]
extern crate treeflection_derive;

use treeflection::{Node, NodeRunner};

#[derive(Node)]
struct Foo {
    bar: u32,
    baz: bool,
}

#[test]
fn test() {
    let mut foo = Foo { bar: 0, baz: true};

    let runner = NodeRunner::new("foo get").unwrap();

    assert_eq!(foo.node_step(runner), String::from("lel"));
}
