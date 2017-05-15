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
fn insert() {
    let expected = vec!(
        NodeToken::Insert,
    );
    assert_command(expected, ":insert");
}

#[test]
fn insert_with_index() {
    let expected = vec!(
        NodeToken::InsertIndex(2),
    );
    assert_command(expected, ":insert 2");
}

#[test]
fn insert_with_key() {
    let expected = vec!(
        NodeToken::InsertKey(String::from("bar")),
    );
    assert_command(expected, ":insert bar");
}

#[test]
fn insert_with_index_key() {
    let expected = vec!(
        NodeToken::InsertIndexKey(2, String::from("bar")),
    );
    assert_command(expected, ":insert 2 bar");
}

#[test]
fn insert_with_index_key_stringed() {
    let expected = vec!(
        NodeToken::InsertIndexKey(2, String::from("bar")),
    );
    assert_command(expected, ":insert \"2\" \"bar\"");
}

#[test]
fn remove() {
    let expected = vec!(
        NodeToken::Remove,
    );
    assert_command(expected, ":remove");
}


#[test]
fn remove_with_index() {
    let expected = vec!(
        NodeToken::RemoveIndex(2),
    );
    assert_command(expected, ":remove 2");
}

#[test]
fn remove_with_key() {
    let expected = vec!(
        NodeToken::RemoveKey(String::from("bar")),
    );
    assert_command(expected, ":remove bar");
}

#[test]
fn set() {
    let expected = vec!(
        NodeToken::Set(String::from("something")),
    );
    assert_command(expected, ":set something");
}

#[test]
fn empty_variant() {
    let expected = vec!(
        NodeToken::SetVariant(String::new()),
    );
    assert_command(expected, ":variant");
}

#[test]
fn variant() {
    let expected = vec!(
        NodeToken::SetVariant(String::from("variant_name")),
    );
    assert_command(expected, ":variant variant_name and trash");
}

#[test]
fn reset() {
    let expected = vec!(
        NodeToken::SetDefault,
    );
    assert_command(expected, ":reset");
}

#[test]
fn set_long_string() {
    let expected = vec!(
        NodeToken::Set(String::from("a single long string containing spaces")),
    );
    assert_command(expected, ":set \"a single long string containing spaces\"");
}

#[test]
fn set_string_escape_quote() {
    let expected = vec!(
        NodeToken::Set(String::from(r#"foo " bar"#)),
    );
    assert_command(expected, r#":set foo \" bar"#);
}

#[test]
fn copy() {
    let expected = vec!(
        NodeToken::CopyFrom,
    );
    assert_command(expected, ":copy");
}

#[test]
fn paste() {
    let expected = vec!(
        NodeToken::PasteTo,
    );
    assert_command(expected, ":paste");
}

#[test]
fn edit() {
    let expected = vec!(
        NodeToken::Edit,
    );
    assert_command(expected, ":edit");
}

#[test]
fn help() {
    let expected = vec!(
        NodeToken::Help,
    );
    assert_command(expected, ":help");
}

/* 
 * Path
 */

#[test]
fn chain_index() {
    let expected = vec!(
        NodeToken::Get,
        NodeToken::ChainIndex(13),
    );
    assert_command(expected, "[13]:get");
}

#[test]
fn chain_key() {
    let expected = vec!(
        NodeToken::Get,
        NodeToken::ChainKey(String::from("key")),
    );
    assert_command(expected, r#"["key"]:get"#);
}

#[test]
fn chain_key_empty() {
    let expected = vec!(
        NodeToken::Get,
        NodeToken::ChainKey(String::from("")),
    );
    assert_command(expected, r#"[""]:get"#);
}

#[test]
fn chain_context() {
    let expected = vec!(
        NodeToken::Get,
        NodeToken::ChainContext,
    );
    assert_command(expected, "[?]:get");
}

#[test]
fn chain_all() {
    let expected = vec!(
        NodeToken::Get,
        NodeToken::ChainAll,
    );
    assert_command(expected, "[*]:get");
}

#[test]
fn property1() {
    let expected = vec!(
        NodeToken::Get,
        NodeToken::ChainProperty(String::from("foo")),
    );
    assert_command(expected, "foo:get");
}

#[test]
fn property1_dot() {
    let expected = vec!(
        NodeToken::Get,
        NodeToken::ChainProperty(String::from("foo")),
    );
    assert_command(expected, ".foo:get");
}

#[test]
fn property2() {
    let expected = vec!(
        NodeToken::Get,
        NodeToken::ChainProperty(String::from("bar")),
        NodeToken::ChainProperty(String::from("foo")),
    );
    assert_command(expected, "foo.bar:get");
}

#[test]
fn space_around_colon() {
    let expected = vec!(
        NodeToken::Get,
        NodeToken::ChainProperty(String::from("foo")),
    );
    assert_command(expected.clone(), "foo:get");
    assert_command(expected.clone(), "foo: get");

    let expected = vec!(
        NodeToken::Get,
        NodeToken::ChainProperty(String::from("foo ")),
    );
    assert_command(expected.clone(), "foo :get");
    assert_command(expected.clone(), "foo : get");
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
        NodeToken::ChainKey(String::from("413")),
        NodeToken::ChainKey(String::from("strings")),
        NodeToken::ChainProperty(String::from("arbitrary")),
        NodeToken::ChainKey(String::from("more")),
        NodeToken::ChainIndex(3),
        NodeToken::ChainIndex(2),
        NodeToken::ChainProperty(String::from("baz")),
        NodeToken::ChainProperty(String::from("bar")),
        NodeToken::ChainProperty(String::from("foo")),
    );
    assert_command(expected, "foo.bar.baz[2][3][\"more\"].arbitrary[\"strings\"][\"413\"][?].almost[?][?][9999].final:get");
}

/*
 * Invalid input handling
 */

fn assert_command_fail(expected_message: &str, command: &str) {
    match NodeRunner::new(command) {
        Ok(_) => {
            panic!("Command is supposed to return Err(_)");
        }
        Err(message) => {
            assert_eq!(expected_message, message);
        }
    }
}

#[test]
fn invalid_inputs() {
    assert_command_fail("Empty property", ".:get");
    assert_command_fail("Empty property", "..:get");
    assert_command_fail("Missing index", "[]:get");
    assert_command_fail("Missing ]", "[:get");
    assert_command_fail("Missing \"]", r#"[":get"#);
    assert_command_fail("Missing \"]", r#"["":get"#);
    assert_command_fail("Missing \"]", r#"["]:get"#);
    assert_command_fail("Invalid index: a", r#"[a]:get"#);
    assert_command_fail("Missing ]", "[?:get");
    assert_command_fail("Missing .", "a[0]a:get");

    // Same as before with missing action
    assert_command_fail("Missing action", ".");
    assert_command_fail("Empty property", "..");
    assert_command_fail("Missing action", "[]");
    assert_command_fail("Missing action", "[");
    assert_command_fail("Missing action", r#"[""#);
    assert_command_fail("Missing action", r#"["""#);
    assert_command_fail("Missing action", r#"["]"#);
    assert_command_fail("Missing action", r#"[#]"#);
    assert_command_fail("Missing action", r#"[a]"#);
    assert_command_fail("Missing ]", "[?");
    assert_command_fail("Missing .", "a[0]a");

    assert_command_fail("Missing action", "[?]");
    assert_command_fail("Empty command", "");
    assert_command_fail("Missing action", r#"foo:"#);
    assert_command_fail("Invalid action", r#"foo:a"#);
}
