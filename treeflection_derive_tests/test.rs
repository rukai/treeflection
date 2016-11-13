// TODO: These tests are in this crate as a workaround for testing treeflection_derive
// while tests folder does not work with macros 1.1 https://github.com/rust-lang/rust/issues/37480
// alternatively we could move the other tests into this crate

#![feature(proc_macro)]

extern crate treeflection;
#[macro_use] extern crate treeflection_derive;
#[macro_use] extern crate matches;
#[macro_use] extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use treeflection::{Node, NodeRunner, NodeToken};

#[derive(Node, Serialize, Deserialize)]
struct Parent {
    pub foo: String,
    pub bar: u32,
    pub baz: bool,
    pub child: Child,
    private: i64,
}

#[derive(Node, Serialize, Deserialize)]
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
fn get_struct() {
    assert_eq!(
        Parent::new().node_step(NodeRunner { tokens: vec!(NodeToken::Get) }),
        String::from(r#"{"foo":"hiya","bar":42,"baz":true,"child":{"qux":-13},"private":1337}"#)
    );
}

#[test]
fn set_struct() {
    let mut parent = Parent::new();
    let runner = NodeRunner { tokens: vec!( NodeToken::Set(
        String::from(r#"{"foo":"Memes","bar":42,"baz":true,"child":{"qux":1337},"private":-1}"#)
    ) )};
    assert_eq!(parent.node_step(runner), String::from(""));
    assert_eq!(parent.foo, String::from("Memes"));
    assert_eq!(parent.bar, 42);
    assert_eq!(parent.baz, true);
    assert_eq!(parent.child.qux, 1337);
    assert_eq!(parent.private, -1);
}

#[test]
fn copy_struct() {
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

#[derive(Node, Serialize, Deserialize)]
enum SomeEnum {
    Foo,
    Bar,
    Baz {x: f32, y: f32},
    Qux (u8),
    Quux (i64, String, bool),
}

#[test]
fn get_unit_enum() {
    let mut some_enum = SomeEnum::Foo;
    let runner = NodeRunner { tokens: vec!(NodeToken::Get) };
    assert_eq!(some_enum.node_step(runner), "\"Foo\"");

    let mut some_enum = SomeEnum::Bar;
    let runner = NodeRunner { tokens: vec!(NodeToken::Get) };
    assert_eq!(some_enum.node_step(runner), "\"Bar\"");
}

#[test]
fn set_unit_enum() {
    let mut some_enum = SomeEnum::Bar;
    let runner = NodeRunner { tokens: vec!( NodeToken::Set(String::from("\"Foo\"")) )};
    assert_eq!(some_enum.node_step(runner), String::from(""));
    assert!(matches!(some_enum, SomeEnum::Foo));

    let mut some_enum = SomeEnum::Bar;
    let runner = NodeRunner { tokens: vec!( NodeToken::Set(String::from("\"Bar\"")) )};
    assert_eq!(some_enum.node_step(runner), String::from(""));
    assert!(matches!(some_enum, SomeEnum::Bar));

    let mut some_enum = SomeEnum::Foo;
    let runner = NodeRunner { tokens: vec!( NodeToken::Set(String::from("\"Bar\"")) )};
    assert_eq!(some_enum.node_step(runner), String::from(""));
    assert!(matches!(some_enum, SomeEnum::Bar));

    let mut some_enum = SomeEnum::Foo;
    let runner = NodeRunner { tokens: vec!( NodeToken::Set(String::from("\"Foo\"")) )};
    assert_eq!(some_enum.node_step(runner), String::from(""));
    assert!(matches!(some_enum, SomeEnum::Foo));

    let mut some_enum = SomeEnum::Foo;
    let runner = NodeRunner { tokens: vec!( NodeToken::Set(String::from("\"Aether\"")) )};
    assert_eq!(some_enum.node_step(runner), "SomeEnum set Error: unknown variant \"Aether\" at line 1 column 8");
    assert!(matches!(some_enum, SomeEnum::Foo));
}

#[test]
fn get_tuple_enum() {
    let mut some_enum = SomeEnum::Qux(42);
    let runner = NodeRunner { tokens: vec!(NodeToken::Get) };
    assert_eq!(some_enum.node_step(runner), "{\"Qux\":42}");

    let mut some_enum = SomeEnum::Quux(-1337, String::from("YOYOYO"), true);
    let runner = NodeRunner { tokens: vec!(NodeToken::Get) };
    assert_eq!(some_enum.node_step(runner), "{\"Quux\":[-1337,\"YOYOYO\",true]}");
}

#[test]
fn set_tuple_enum() {
    let mut some_enum = SomeEnum::Foo;
    let runner = NodeRunner { tokens: vec!(NodeToken::Set(String::from("{\"Qux\":13}"))) };
    assert_eq!(some_enum.node_step(runner), String::from(""));
    assert!(matches!(some_enum, SomeEnum::Qux(13)));

    let some_string = String::from("SomeString");
    let mut some_enum = SomeEnum::Bar;
    let runner = NodeRunner { tokens: vec!(NodeToken::Set(String::from("{\"Quux\":[-42, \"SomeString\", true]}"))) };
    assert_eq!(some_enum.node_step(runner), String::from(""));
    assert!(matches!(some_enum, SomeEnum::Quux(-42, some_string, true))); // TODO: I suspect this isnt testing properly due to the unused variable warning ...
}

#[test]
fn get_struct_enum() {
    let mut some_enum = SomeEnum::Baz {x: 412.12345, y: 44.11};
    let runner = NodeRunner { tokens: vec!(NodeToken::Get) };
    assert_eq!(some_enum.node_step(runner), r#"{"Baz":{"x":412.12344,"y":44.11}}"#);
}

#[test]
fn set_struct_enum() {
    let mut some_enum = SomeEnum::Baz {x: 412.12345, y: 44.11};
    let runner = NodeRunner { tokens: vec!(NodeToken::Set(String::from(r#"{"Baz":{"x":1337.1337,"y":42.13}}"#))) };
    assert_eq!(some_enum.node_step(runner), String::from(""));
    assert!(matches!(some_enum, SomeEnum::Baz {x: 1337.1337, y: 42.13}));
}

#[test]
fn copy_enum() {
    let mut some_enum = SomeEnum::Foo;
    let runner = NodeRunner { tokens: vec!(NodeToken::CopyFrom) };
    assert_eq!(some_enum.node_step(runner), String::from("SomeEnum cannot 'CopyFrom'"));
}
