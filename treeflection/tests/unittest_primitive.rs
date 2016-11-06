extern crate treeflection;

use treeflection::{Node, NodeRunner, NodeToken};
use std::fmt::Debug;

fn assert_set<T>(mut node: T, set: &str, expected: T) where T: Node + Debug + PartialEq {
    let tokens = vec!(NodeToken::Set(String::from(set)));
    node.node_step(NodeRunner { tokens: tokens });
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
fn i64_copy_message() {
    let mut node: i64 = 5;
    let tokens = vec!(NodeToken::CopyFrom);
    assert_eq!("i64 cannot 'CopyFrom'", node.node_step(NodeRunner { tokens: tokens }));
}

#[test]
fn float_set() {
    assert_set::<f32>(49992.12345, "3.141592653589793", 3.141592653589793);
    assert_set::<f64>(49992.12345, "3.141592653589793", 3.141592653589793);
}

#[test]
fn float_get() {
    assert_get::<f32>(6.283185307179586, "6.2831855");
    assert_get::<f64>(6.283185307179586, "6.283185307179586");
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
