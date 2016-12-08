// Test NodeRunner::new directly

extern crate treeflection;

use treeflection::{NodeRunner, NodeToken};

fn assert_command(expected: Vec<NodeToken>, command: &str) {
    let runner = NodeRunner::new(command).unwrap();
    assert_eq!(expected, runner.tokens);
}

/* 
 * Actions
 */
#[test]
fn insert_with_index() {
    let expected = vec!(
        NodeToken::Insert(2),
        NodeToken::ChainProperty(String::from("foo")),
    );
    assert_command(expected, "foo insert 2");
}

#[test]
fn insert() {
    let expected = vec!(
        NodeToken::Insert(0),
        NodeToken::ChainProperty(String::from("foo")),
    );
    assert_command(expected, "foo insert");
}

#[test]
fn remove_with_index() {
    let expected = vec!(
        NodeToken::Remove(2),
        NodeToken::ChainProperty(String::from("foo")),
    );
    assert_command(expected, "foo remove 2");
}

#[test]
fn remove() {
    let expected = vec!(
        NodeToken::Remove(0),
        NodeToken::ChainProperty(String::from("foo")),
    );
    assert_command(expected, "foo remove");
}

#[test]
fn set() {
    let expected = vec!(
        NodeToken::Set(String::from("something")),
        NodeToken::ChainProperty(String::from("foo")),
    );
    assert_command(expected, "foo set something");
}

#[test]
fn variant() {
    let expected = vec!(
        NodeToken::SetVariant(String::from("variant_name")),
        NodeToken::ChainProperty(String::from("foo")),
    );
    assert_command(expected, "foo variant variant_name and trash");
}

#[test]
fn reset() {
    let expected = vec!(
        NodeToken::SetDefault,
        NodeToken::ChainProperty(String::from("foo")),
    );
    assert_command(expected, "foo reset");
}

#[test]
fn set_with_space() {
    let expected = vec!(
        NodeToken::Set(String::from("something with space")),
        NodeToken::ChainProperty(String::from("foo")),
    );
    assert_command(expected, "foo set something with space");
}

#[test]
fn copy() {
    let expected = vec!(
        NodeToken::CopyFrom,
        NodeToken::ChainProperty(String::from("foo")),
    );
    assert_command(expected, "foo copy");
}

#[test]
fn paste() {
    let expected = vec!(
        NodeToken::PasteTo,
        NodeToken::ChainProperty(String::from("foo")),
    );
    assert_command(expected, "foo paste");
}

#[test]
fn edit() {
    let expected = vec!(
        NodeToken::Edit,
        NodeToken::ChainProperty(String::from("foo")),
    );
    assert_command(expected, "foo edit");
}

#[test]
fn help() {
    let expected = vec!(
        NodeToken::Help,
        NodeToken::ChainProperty(String::from("foo")),
    );
    assert_command(expected, "foo help");
}

/* 
 * Path
 */

#[test]
fn chain_index() {
    let expected = vec!(
        NodeToken::Get,
        NodeToken::ChainIndex(13),
        NodeToken::ChainProperty(String::from("foo")),
    );
    assert_command(expected, "foo[13] get");
}

#[test]
fn chain_context() {
    let expected = vec!(
        NodeToken::Get,
        NodeToken::ChainKey(String::from("key")),
        NodeToken::ChainProperty(String::from("foo")),
    );
    assert_command(expected, "foo[key] get");
}

#[test]
fn chain_key() {
    let expected = vec!(
        NodeToken::Get,
        NodeToken::ChainContext,
        NodeToken::ChainProperty(String::from("foo")),
    );
    assert_command(expected, "foo[?] get");
}

#[test]
fn complex_path() {
    let expected = vec!(
        NodeToken::Get,
        NodeToken::ChainProperty(String::from("final")),
        NodeToken::ChainIndex(9999),
        NodeToken::ChainContext,
        NodeToken::ChainContext,
        NodeToken::ChainProperty(String::from("almost")),
        NodeToken::ChainContext,
        NodeToken::ChainKey(String::from("strings")),
        NodeToken::ChainProperty(String::from("arbitrary")),
        NodeToken::ChainKey(String::from("more")),
        NodeToken::ChainIndex(3),
        NodeToken::ChainIndex(2),
        NodeToken::ChainProperty(String::from("baz")),
        NodeToken::ChainProperty(String::from("bar")),
        NodeToken::ChainProperty(String::from("foo")),
    );
    assert_command(expected, "foo.bar.baz[2][3][more].arbitrary[strings][?].almost[?][?][9999].final get");
}
