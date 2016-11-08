// TODO: These tests are in this crate as a workaround for testing treeflection_derive
// while tests folder does not work with macros 1.1 https://github.com/rust-lang/rust/issues/37480
// alternatively we could move the other tests into this crate

#![feature(proc_macro)]

extern crate treeflection;
#[macro_use] extern crate treeflection_derive;
#[macro_use] extern crate matches;

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
fn get_struct() {
    let runner = NodeRunner { tokens: vec!(NodeToken::Get) };
    assert_eq!(Parent::new().node_step(runner), String::from("This is a struct"));
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

#[derive(Node)]
enum SomeEnum {
    Foo,
    Bar,
    //Baz {x: f32, y: f32},
    Qux (u8),
    Quux (i64, String, bool),
}

//impl Node for SomeEnum {
//    fn node_step ( & mut self , mut runner : NodeRunner ) -> String {
//        match runner . step ( ) {
//            NodeToken :: Get => {
//                match self {
//                    &mut SomeEnum :: Foo => String::from("Foo") ,
//                    &mut SomeEnum :: Bar => String::from("Bar") ,
//                    &mut SomeEnum :: Qux(ref v1) => format!("Qux({})", v1) ,
//                    &mut SomeEnum :: Quux(ref v1, ref v2, ref v3) => format!("Quux({}, {}, {})", v1, v2, v3) ,
//                }
//            }
//            NodeToken :: Set ( value ) => {
//                match value.as_ref() {
//                    "Foo" => { *self = SomeEnum :: Foo; String::from("") } ,
//                    "Bar" => { *self = SomeEnum :: Bar; String::from("") } ,
//                    value_miss => { format!("{} is not a valid value for {}", value_miss, "SomeEnum") },
//                }
//            }
//            action => { format ! ( "{} cannot '{:?}'" , "SomeEnum" , action ) }
//        }
//    }
//}

#[test]
fn get_unit_enum() {
    let mut some_enum = SomeEnum::Foo;
    let runner = NodeRunner { tokens: vec!(NodeToken::Get) };
    assert_eq!(some_enum.node_step(runner), "Foo");

    let mut some_enum = SomeEnum::Bar;
    let runner = NodeRunner { tokens: vec!(NodeToken::Get) };
    assert_eq!(some_enum.node_step(runner), "Bar");
}

#[test]
fn set_unit_enum() {
    let mut some_enum = SomeEnum::Bar;
    let runner = NodeRunner { tokens: vec!( NodeToken::Set(String::from("Foo")) )};
    some_enum.node_step(runner);
    assert!(matches!(some_enum, SomeEnum::Foo));

    let mut some_enum = SomeEnum::Bar;
    let runner = NodeRunner { tokens: vec!( NodeToken::Set(String::from("Bar")) )};
    some_enum.node_step(runner);
    assert!(matches!(some_enum, SomeEnum::Bar));

    let mut some_enum = SomeEnum::Foo;
    let runner = NodeRunner { tokens: vec!( NodeToken::Set(String::from("Bar")) )};
    some_enum.node_step(runner);
    assert!(matches!(some_enum, SomeEnum::Bar));

    let mut some_enum = SomeEnum::Foo;
    let runner = NodeRunner { tokens: vec!( NodeToken::Set(String::from("Foo")) )};
    some_enum.node_step(runner);
    assert!(matches!(some_enum, SomeEnum::Foo));

    let mut some_enum = SomeEnum::Foo;
    let runner = NodeRunner { tokens: vec!( NodeToken::Set(String::from("Aether")) )};
    assert_eq!(some_enum.node_step(runner), "Aether is not a valid value for SomeEnum");
    assert!(matches!(some_enum, SomeEnum::Foo));
}

#[test]
fn get_tuple_enum() {
    let mut some_enum = SomeEnum::Qux(42);
    let runner = NodeRunner { tokens: vec!(NodeToken::Get) };
    assert_eq!(some_enum.node_step(runner), "Qux(42)");

    let mut some_enum = SomeEnum::Quux(-1337, String::from("YOYOYO"), true);
    let runner = NodeRunner { tokens: vec!(NodeToken::Get) };
    assert_eq!(some_enum.node_step(runner), "Quux(-1337, YOYOYO, true)");
}

#[test]
fn set_tuple_enum() {
}

#[test]
fn get_struct_enum() {
}

#[test]
fn set_struct_enum() {
}

#[test]
fn copy_enum() {
    let mut some_enum = SomeEnum::Foo;
    let runner = NodeRunner { tokens: vec!(NodeToken::CopyFrom) };
    assert_eq!(some_enum.node_step(runner), String::from("SomeEnum cannot 'CopyFrom'"));
}
