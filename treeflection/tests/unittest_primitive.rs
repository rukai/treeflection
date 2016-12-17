extern crate treeflection;

use treeflection::{Node, NodeRunner, NodeToken};
use std::fmt::Debug;

fn assert_set<T>(mut node: T, set: &str, expected: T) where T: Node + Debug + PartialEq {
    let tokens = vec!(NodeToken::Set(String::from(set)));
    assert_eq!(node.node_step(NodeRunner { tokens: tokens }), String::new());
    assert_eq!(expected, node);
}

fn assert_set_output<T>(mut node: T, set: &str, expected: T, expected_output: &str) where T: Node + Debug + PartialEq {
    let tokens = vec!(NodeToken::Set(String::from(set)));
    assert_eq!(node.node_step(NodeRunner { tokens: tokens }), expected_output);
    assert_eq!(expected, node);
}

fn assert_get<T: Node>(mut node: T, expected: &str) {
    let tokens = vec!(NodeToken::Get);
    let result = node.node_step(NodeRunner { tokens: tokens });
    assert_eq!(expected, String::from(result));
}

#[test]
fn int_set() {
    assert_set::<isize>(42, "13", 13);
    assert_set::<i64>(42, "13", 13);
    assert_set::<i32>(42, "13", 13);
    assert_set::<i16>(42, "13", 13);
    assert_set::<i8>(42, "13", 13);

    assert_set::<isize>(42, "-19", -19);
    assert_set::<i64>(42, "-19", -19);
    assert_set::<i32>(42, "-19", -19);
    assert_set::<i16>(42, "-19", -19);
    assert_set::<i8>(42, "-19", -19);

    assert_set::<usize>(42, "13", 13);
    assert_set::<u64>(42, "13", 13);
    assert_set::<u32>(42, "13", 13);
    assert_set::<u16>(42, "13", 13);
    assert_set::<u8>(42, "13", 13);

    assert_set_output::<isize>(42, "invalid", 42, "Invalid value for isize (needs to be: A number from –9,223,372,036,854,775,808 to 9,223,372,036,854,775,807)");
    assert_set_output::<i64>(42, "invalid", 42, "Invalid value for i64 (needs to be: A number from –9,223,372,036,854,775,808 to 9,223,372,036,854,775,807)");
    assert_set_output::<i32>(42, "invalid", 42, "Invalid value for i32 (needs to be: A number from –2,147,483,648 to 2,147,483,647)");
    assert_set_output::<i16>(42, "invalid", 42, "Invalid value for i16 (needs to be: A number from –32,768 to –32,767)");
    assert_set_output::<i8>(42, "invalid", 42, "Invalid value for i8 (needs to be: A number from -128 to 127)");

    assert_set_output::<usize>(42, "invalid", 42, "Invalid value for usize (needs to be: A number from 0 to 18,446,744,073,709,551,615)");
    assert_set_output::<u64>(42, "invalid", 42, "Invalid value for u64 (needs to be: A number from 0 to 18,446,744,073,709,551,615)");
    assert_set_output::<u32>(42, "invalid", 42, "Invalid value for u32 (needs to be: A number from 0 to 4,294,967,295)");
    assert_set_output::<u16>(42, "invalid", 42, "Invalid value for u16 (needs to be: A number from 0 to 65,535)");
    assert_set_output::<u8>(42, "invalid", 42, "Invalid value for u8 (needs to be: A number from 0 to 255)");
}

#[test]
fn int_get() {
    assert_get::<isize>(42, "42");
    assert_get::<i64>(42, "42");
    assert_get::<i32>(42, "42");
    assert_get::<i16>(42, "42");
    assert_get::<i8>(42, "42");

    assert_get::<isize>(-4, "-4");
    assert_get::<i64>(-4, "-4");
    assert_get::<i32>(-4, "-4");
    assert_get::<i16>(-4, "-4");
    assert_get::<i8>(-4, "-4");

    assert_get::<usize>(42, "42");
    assert_get::<u64>(42, "42");
    assert_get::<u32>(42, "42");
    assert_get::<u16>(42, "42");
    assert_get::<u8>(42, "42");
}

#[test]
fn int_help() {
    let runner = NodeRunner { tokens: vec!( NodeToken::Help ) };

    let output = r#"
u8 Help

Valid values: A number from 0 to 255

Commands:
*   help - display this help
*   get  - display value
*   set  - set to value"#;
    let mut value: u8 = 13;
    assert_eq!(value.node_step(runner.clone()).as_str(), output);

    let output = r#"
u16 Help

Valid values: A number from 0 to 65,535

Commands:
*   help - display this help
*   get  - display value
*   set  - set to value"#;
    let mut value: u16 = 13;
    assert_eq!(value.node_step(runner.clone()).as_str(), output);

    let output = r#"
u32 Help

Valid values: A number from 0 to 4,294,967,295

Commands:
*   help - display this help
*   get  - display value
*   set  - set to value"#;
    let mut value: u32 = 13;
    assert_eq!(value.node_step(runner.clone()).as_str(), output);

    let output = r#"
u64 Help

Valid values: A number from 0 to 18,446,744,073,709,551,615

Commands:
*   help - display this help
*   get  - display value
*   set  - set to value"#;
    let mut value: u64 = 13;
    assert_eq!(value.node_step(runner.clone()).as_str(), output);

    let output = r#"
i8 Help

Valid values: A number from -128 to 127

Commands:
*   help - display this help
*   get  - display value
*   set  - set to value"#;
    let mut value: i8 = 13;
    assert_eq!(value.node_step(runner.clone()).as_str(), output);

    let output = r#"
i16 Help

Valid values: A number from –32,768 to –32,767

Commands:
*   help - display this help
*   get  - display value
*   set  - set to value"#;
    let mut value: i16 = 13;
    assert_eq!(value.node_step(runner.clone()).as_str(), output);

    let output = r#"
i32 Help

Valid values: A number from –2,147,483,648 to 2,147,483,647

Commands:
*   help - display this help
*   get  - display value
*   set  - set to value"#;
    let mut value: i32 = 13;
    assert_eq!(value.node_step(runner.clone()).as_str(), output);

    let output = r#"
i64 Help

Valid values: A number from –9,223,372,036,854,775,808 to 9,223,372,036,854,775,807

Commands:
*   help - display this help
*   get  - display value
*   set  - set to value"#;
    let mut value: i64 = 13;
    assert_eq!(value.node_step(runner.clone()).as_str(), output);
}

#[test]
fn i64_copy_message() {
    let mut node: i64 = 5;
    let tokens = vec!(NodeToken::CopyFrom);
    assert_eq!("i64 cannot 'CopyFrom'", node.node_step(NodeRunner { tokens: tokens }));
}

#[test]
fn float_set() {
    assert_set::<f32>(49992.12345, "3.141592653589793", 3.141592653589793);
    assert_set::<f64>(49992.12345, "3.141592653589793", 3.141592653589793);

    assert_set_output::<f32>(49992.12345, "invalid", 49992.12345, "Invalid value for f32 (needs to be: A number with a decimal point)");
    assert_set_output::<f64>(49992.12345, "invalid", 49992.12345, "Invalid value for f64 (needs to be: A higher precision number with a decimal point)");
}

#[test]
fn float_get() {
    assert_get::<f32>(6.283185307179586, "6.2831855");
    assert_get::<f64>(6.283185307179586, "6.283185307179586");
}

#[test]
fn float_help() {
    let runner = NodeRunner { tokens: vec!( NodeToken::Help ) };

    let output = r#"
f32 Help

Valid values: A number with a decimal point

Commands:
*   help - display this help
*   get  - display value
*   set  - set to value"#;
    let mut value: f32 = 13.37;
    assert_eq!(value.node_step(runner.clone()).as_str(), output);

    let output = r#"
f64 Help

Valid values: A higher precision number with a decimal point

Commands:
*   help - display this help
*   get  - display value
*   set  - set to value"#;
    let mut value: f64 = 13.37;
    assert_eq!(value.node_step(runner.clone()).as_str(), output);
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
fn string_help() {
    let output = r#"
String Help

Valid values: Anything

Commands:
*   help - display this help
*   get  - display value
*   set  - set to value"#;
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
