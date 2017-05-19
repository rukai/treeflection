#![feature(drop_types_in_const)]

extern crate treeflection;
#[macro_use] extern crate treeflection_derive;
#[macro_use] extern crate matches;
#[macro_use] extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use treeflection::{Node, NodeRunner, NodeToken};

#[derive(Node, Serialize, Deserialize, Default, Clone)]
struct Parent {
    pub foo: String,
    pub bar: u32,
    pub baz: bool,
    pub child: Child,
    private: i64,
}

#[NodeActions(
    NodeAction(action="action_name", function="function_name", args="1", help="add the first argument to qux"),
    NodeAction(function="same_name", return_string),
)]
#[derive(Node, Serialize, Deserialize, Default, Clone)]
struct Child {
    pub qux: i32,
}

impl Child {
    fn new() -> Child {
        Child {
            qux: 413,
        }
    }

    fn function_name(&mut self, value: String) {
        self.qux += value.parse().unwrap();
    }

    fn same_name(&self) -> String {
        String::from("basic action")
    }
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

    fn empty() -> Parent {
        Parent {
            foo: String::new(),
            bar: 0,
            baz: false,
            child: Child {
                qux: 0,
            },
            private: 0,
        }
    }
}

#[test]
fn custom_function_name() {
    let mut child = Child::new();
    let runner = NodeRunner { tokens: vec!(
        NodeToken::Custom(String::from("action_name"), vec!(String::from("7")))
    )};
    assert_eq!(child.node_step(runner), String::from(""));
    assert_eq!(child.qux, 420);
}

#[test]
fn custom_same_name() {
    let mut child = Child::new();
    let runner = NodeRunner { tokens: vec!(
        NodeToken::Custom(String::from("same_name"), vec!())
    )};
    assert_eq!(child.node_step(runner), String::from("basic action"));
    assert_eq!(child.qux, 413);
}

#[test]
fn get_struct() {
    let output =
r#"{
  "foo": "hiya",
  "bar": 42,
  "baz": true,
  "child": {
    "qux": -13
  },
  "private": 1337
}"#;
    assert_eq!(
        Parent::new().node_step(NodeRunner { tokens: vec!(NodeToken::Get) }),
        String::from(output)
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

#[test]
fn default_struct() {
    let runner = NodeRunner { tokens: vec!(NodeToken::SetDefault) };
    let mut parent = Parent::new();
    assert_eq!(parent.node_step(runner), String::from(""));
    assert_eq!(parent.foo, String::new());
    assert_eq!(parent.bar, 0);
    assert_eq!(parent.baz, false);
    assert_eq!(parent.child.qux, 0);
    assert_eq!(parent.private, 0);
}

#[test]
fn variant_struct() {
    let runner = NodeRunner { tokens: vec!(NodeToken::SetVariant(String::from("something"))) };
    let mut parent = Parent::new();
    assert_eq!(parent.node_step(runner), String::from("Parent cannot \'SetVariant(\"something\")\'"));
}

#[test]
fn copy_paste_struct() {
    let copy_token  = NodeRunner { tokens: vec!(NodeToken::CopyFrom) };
    let paste_token = NodeRunner { tokens: vec!(NodeToken::PasteTo) };

    let mut a = Parent::new();
    let mut b = Parent::empty();

    assert_eq!(a.node_step(copy_token), "");
    assert_eq!(a.bar, 42);
    assert_eq!(a.child.qux, -13);

    assert_eq!(b.bar, 0);
    assert_eq!(b.child.qux, 0);
    assert_eq!(b.node_step(paste_token), "");
    assert_eq!(b.bar, 42);
    assert_eq!(a.child.qux, -13);
}

#[test]
fn help_struct_parent() {
    let output = r#"
Parent Help

Actions:
*   help  - display this help
*   get   - display JSON
*   set   - set to JSON
*   copy  - copy the values from this struct
*   paste - paste the copied values to this struct
*   reset - reset to default values

Accessors:
*   foo - String
*   bar - u32
*   baz - bool
*   child - Child"#;
    let mut parent = Parent::new();
    let runner = NodeRunner { tokens: vec!(NodeToken::Help) };
    assert_eq!(parent.node_step(runner), String::from(output));
}

#[test]
fn help_struct_child() {
    let output = r#"
Child Help

Actions:
*   help  - display this help
*   get   - display JSON
*   set   - set to JSON
*   copy  - copy the values from this struct
*   paste - paste the copied values to this struct
*   reset - reset to default values
*   action_name - add the first argument to qux
*   same_name

Accessors:
*   qux - i32"#;
    let mut parent = Child::new();
    let runner = NodeRunner { tokens: vec!(NodeToken::Help) };
    assert_eq!(parent.node_step(runner), String::from(output));
}

#[derive(Node, Serialize, Deserialize, Clone, Debug)]
enum SomeEnum {
    Foo,
    Bar,
    Baz {x: f32, y: f32},
    Qux (u8),
    Quux (i64, String, bool),
    GenericInTuple (Vec<usize>),
    GenericInStruct {generic: Vec<usize>},
}

impl Default for SomeEnum {
    fn default() -> SomeEnum {
        SomeEnum::Foo
    }
}

// test for unused variable warnings in generated code
#[derive(Node, Serialize, Deserialize, Clone)]
enum SimpleEnum {
    Foo,
}

impl Default for SimpleEnum {
    fn default() -> SimpleEnum {
        SimpleEnum::Foo
    }
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
    assert_eq!(some_enum.node_step(runner), "SomeEnum set Error: unknown variant `Aether`, expected one of `Foo`, `Bar`, `Baz`, `Qux`, `Quux`, `GenericInTuple`, `GenericInStruct` at line 1 column 8");
    assert!(matches!(some_enum, SomeEnum::Foo));
}

#[test]
fn get_tuple_enum() {
    let mut some_enum = SomeEnum::Qux(42);
    let runner = NodeRunner { tokens: vec!(NodeToken::Get) };
    let output =
r#"{
  "Qux": 42
}"#;
    assert_eq!(some_enum.node_step(runner), output);

    let mut some_enum = SomeEnum::Quux(-1337, String::from("YOYOYO"), true);
    let runner = NodeRunner { tokens: vec!(NodeToken::Get) };
    let output =
r#"{
  "Quux": [
    -1337,
    "YOYOYO",
    true
  ]
}"#;
    assert_eq!(some_enum.node_step(runner), output);
}

#[test]
fn set_tuple_enum() {
    let mut some_enum = SomeEnum::Foo;
    let runner = NodeRunner { tokens: vec!(NodeToken::Set(String::from("{\"Qux\":13}"))) };
    assert_eq!(some_enum.node_step(runner), String::from(""));
    assert!(matches!(some_enum, SomeEnum::Qux(13)));

    let mut some_enum = SomeEnum::Bar;
    let runner = NodeRunner { tokens: vec!(NodeToken::Set(String::from("{\"Quux\":[-42, \"SomeString\", true]}"))) };
    assert_eq!(some_enum.node_step(runner), String::from(""));
    match some_enum {
        SomeEnum::Quux (-42, some_string, true) => {
            assert_eq!(some_string.as_str(), "SomeString");
        }
        _ => { panic!("Did not match SomeEnum::Quux (-42, _, true)") }
    }
}

#[test]
fn get_struct_enum() {
    let mut some_enum = SomeEnum::Baz {x: 412.12345, y: 44.11};
    let runner = NodeRunner { tokens: vec!(NodeToken::Get) };
    let output =
r#"{
  "Baz": {
    "x": 412.12344,
    "y": 44.11
  }
}"#;
    assert_eq!(some_enum.node_step(runner), output);
}

#[test]
fn set_struct_enum() {
    let mut some_enum = SomeEnum::Baz {x: 412.12345, y: 44.11};
    let runner = NodeRunner { tokens: vec!(NodeToken::Set(String::from(r#"{"Baz":{"x":1337.1337,"y":42.13}}"#))) };
    assert_eq!(some_enum.node_step(runner), String::from(""));
    assert_eq!(format!("{:?}", some_enum), String::from("Baz { x: 1337.1337, y: 42.13 }"));
}

#[test]
fn no_property_unit_enum() {
    let mut some_enum = SomeEnum::Foo;
    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainProperty(String::from("notx")),
    )};
    assert_eq!(some_enum.node_step(runner), String::from("Foo does not have a property 'notx'"));
}

#[test]
fn no_property_tuple_enum() {
    let mut some_enum = SomeEnum::Qux(42);
    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainProperty(String::from("notx")),
    )};
    assert_eq!(some_enum.node_step(runner), String::from("Qux does not have a property 'notx'"));
}

#[test]
fn no_property_struct_enum() {
    let mut some_enum = SomeEnum::Baz { x: 42.0, y: 13.37 };
    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainProperty(String::from("notx")),
    )};
    assert_eq!(some_enum.node_step(runner), String::from("Baz does not have a property 'notx'"));
}

#[test]
fn f32_property_struct_enum() {
    let mut some_enum = SomeEnum::Baz { x: 42.0, y: 13.37 };
    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainProperty(String::from("x")),
    )};
    assert_eq!(some_enum.node_step(runner), String::from("42"));

    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainProperty(String::from("y")),
    )};
    assert_eq!(some_enum.node_step(runner), String::from("13.37"));
}

#[test]
fn index_unit_enum() {
    let mut some_enum = SomeEnum::Foo;
    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainIndex(0),
    )};
    assert_eq!(some_enum.node_step(runner), String::from("Cannot index Foo"));
}

#[test]
fn index_struct_enum() {
    let mut some_enum = SomeEnum::Baz { x: 42.0, y: 13.37 };
    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainIndex(0),
    )};
    assert_eq!(some_enum.node_step(runner), String::from("Cannot index Baz"));
}

#[test]
fn index_tuple_enum() {
    let mut some_enum = SomeEnum::Quux(-1337, String::from("YOYOYO"), true);
    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainIndex(0),
    )};
    assert_eq!(some_enum.node_step(runner), String::from("-1337"));

    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainIndex(1),
    )};
    assert_eq!(some_enum.node_step(runner), String::from("YOYOYO"));

    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainIndex(2),
    )};
    assert_eq!(some_enum.node_step(runner), String::from("true"));

    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainIndex(3),
    )};
    assert_eq!(some_enum.node_step(runner), String::from("Used index 3 on a Quux (try a value between 0-2"));
}

#[test]
fn variant_enum() {
    let mut some_enum = SomeEnum::Bar;
    let runner = NodeRunner { tokens: vec!(NodeToken::SetVariant(String::from("Foo"))) };
    assert_eq!(some_enum.node_step(runner), String::from(""));
    assert!(matches!(some_enum, SomeEnum::Foo));

    let runner = NodeRunner { tokens: vec!(NodeToken::SetVariant(String::from("Baz"))) };
    assert_eq!(some_enum.node_step(runner), String::from(""));
    assert_eq!(format!("{:?}", some_enum), String::from("Baz { x: 0, y: 0 }"));

    let runner = NodeRunner { tokens: vec!(NodeToken::SetVariant(String::from("Qux"))) };
    assert_eq!(some_enum.node_step(runner), String::from(""));
    assert!(matches!(some_enum, SomeEnum::Qux (0)));

    let mut some_enum = SomeEnum::Bar;
    let runner = NodeRunner { tokens: vec!(NodeToken::SetVariant(String::from("nonexistent"))) };
    assert_eq!(some_enum.node_step(runner), String::from("SomeEnum does not have a variant 'nonexistent'"));
    assert!(matches!(some_enum, SomeEnum::Bar));
}

#[test]
fn default_enum() {
    let mut some_enum = SomeEnum::Bar;
    let runner = NodeRunner { tokens: vec!(NodeToken::SetDefault) };
    assert_eq!(some_enum.node_step(runner), String::from(""));
    assert!(matches!(some_enum, SomeEnum::Foo));
}

#[test]
fn copy_paste_enum() {
    let copy_token  = NodeRunner { tokens: vec!(NodeToken::CopyFrom) };
    let paste_token = NodeRunner { tokens: vec!(NodeToken::PasteTo) };

    let mut a = SomeEnum::Qux (13);
    let mut b = SomeEnum::Foo;

    assert_eq!(a.node_step(copy_token), "");
    assert!(matches!(a, SomeEnum::Qux (13)));

    assert_eq!(b.node_step(paste_token), "");
    assert!(matches!(b, SomeEnum::Qux (13)));
}

// TODO: display tuple and struct enum details under valid values:
// Probably use json equivalent of below
//*   Foo
//*   Bar
//*   Baz {x: f32, y: f32}
//*   Qux (u8)
//*   Quux (i64, String, bool)
#[test]
fn help_enum() {
    let output = r#"
SomeEnum Help

Actions:
*   help    - display this help
*   get     - display JSON
*   set     - set to JSON
*   copy    - copy the values from this enum
*   paste   - paste the copied values to this enum
*   reset   - reset to default variant
*   variant - set to the specified variant

Valid variants:
*   Foo
*   Bar
*   Baz
*   Qux
*   Quux
*   GenericInTuple
*   GenericInStruct

Accessors:
Changes depending on which variant the enum is currently set to:

As Baz:
*   .x - f32
*   .y - f32
As Qux:
*   [0] - u8
As Quux:
*   [0] - i64
*   [1] - String
*   [2] - bool
As GenericInTuple:
*   [0] - Vec
As GenericInStruct:
*   .generic - Vec
"#;
    let mut some_enum = SomeEnum::Foo;
    let runner = NodeRunner { tokens: vec!(NodeToken::Help) };
    assert_eq!(some_enum.node_step(runner), String::from(output));
}
