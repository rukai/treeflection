extern crate treeflection;

use std::collections::HashMap;

use treeflection::{Node, NodeRunner, NodeToken};

fn test_map4() -> HashMap<String, i32> {
    let mut map = HashMap::new();
    map.insert(String::from("foo"), 100000);
    map.insert(String::from("bar"), 13);
    map.insert(String::from("baz"), -358);
    map.insert(String::from("qux"), 42);
    map
}

fn test_map1() -> HashMap<String, i32> {
    let mut map = HashMap::new();
    map.insert(String::from("foo"), 13);
    map
}

fn test_map0() -> HashMap<String, i32> {
    HashMap::new()
}

#[test]
fn map_chain_all() {
    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainAll,
    )};
    assert_eq!("|13|-358|100000|42|", test_map4().node_step(runner));
}

#[test]
fn map_chain_key() {
    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainKey(String::from("foo")),
    )};
    assert_eq!("100000", test_map4().node_step(runner));

    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainKey(String::from("bar")),
    )};
    assert_eq!("13", test_map4().node_step(runner));

    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainKey(String::from("baz")),
    )};
    assert_eq!("-358", test_map4().node_step(runner));

    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainKey(String::from("qux")),
    )};
    assert_eq!("42", test_map4().node_step(runner));

    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainKey(String::from("quux")),
    )};
    assert_eq!(test_map4().node_step(runner), "Used key 'quux' on a map that does not contain it. Try one of: 'bar', 'baz', 'foo', 'qux'");

    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainKey(String::from("map")),
    )};
    assert_eq!(test_map0().node_step(runner), "Used key 'map' on an empty map.");
}

#[test]
fn map_insert() {
    let mut some_map = test_map4();

    assert_eq!(some_map.len(), 4);
    assert_eq!(*some_map.get("foo").unwrap(), 100000);
    assert_eq!(*some_map.get("bar").unwrap(), 13);
    assert_eq!(*some_map.get("baz").unwrap(), -358);
    assert_eq!(*some_map.get("qux").unwrap(), 42);

    let runner = NodeRunner { tokens: vec!(NodeToken::InsertKey(String::from("qux"))) };
    assert_eq!(some_map.node_step(runner), "Tried to insert key 'qux' on a map that already contains it. Current keys: 'bar', 'baz', 'foo', 'qux'");
    assert_eq!(some_map.len(), 4);
    assert_eq!(*some_map.get("foo").unwrap(), 100000);
    assert_eq!(*some_map.get("bar").unwrap(), 13);
    assert_eq!(*some_map.get("baz").unwrap(), -358);
    assert_eq!(*some_map.get("qux").unwrap(), 42);

    let runner = NodeRunner { tokens: vec!(NodeToken::InsertKey(String::from("quux"))) };
    assert_eq!(some_map.node_step(runner), "");
    assert_eq!(some_map.len(), 5);
    assert_eq!(*some_map.get("foo").unwrap(), 100000);
    assert_eq!(*some_map.get("bar").unwrap(), 13);
    assert_eq!(*some_map.get("baz").unwrap(), -358);
    assert_eq!(*some_map.get("qux").unwrap(), 42);
    assert_eq!(*some_map.get("quux").unwrap(), 0);
}

#[test]
fn map_remove() {
    let mut some_map = test_map4();

    assert_eq!(some_map.len(), 4);
    assert_eq!(*some_map.get("foo").unwrap(), 100000);
    assert_eq!(*some_map.get("bar").unwrap(), 13);
    assert_eq!(*some_map.get("baz").unwrap(), -358);
    assert_eq!(*some_map.get("qux").unwrap(), 42);

    let runner = NodeRunner { tokens: vec!(NodeToken::RemoveKey(String::from("quux"))) };
    assert_eq!(some_map.node_step(runner), "Tried to remove key 'quux' on a map that doesnt contain it. Current keys: 'bar', 'baz', 'foo', 'qux'");
    assert_eq!(some_map.len(), 4);
    assert_eq!(*some_map.get("foo").unwrap(), 100000);
    assert_eq!(*some_map.get("bar").unwrap(), 13);
    assert_eq!(*some_map.get("baz").unwrap(), -358);
    assert_eq!(*some_map.get("qux").unwrap(), 42);

    let runner = NodeRunner { tokens: vec!(NodeToken::RemoveKey(String::from("foo"))) };
    assert_eq!("", some_map.node_step(runner));
    assert_eq!(some_map.len(), 3);
    assert_eq!(*some_map.get("bar").unwrap(), 13);
    assert_eq!(*some_map.get("baz").unwrap(), -358);
    assert_eq!(*some_map.get("qux").unwrap(), 42);
}

#[test]
fn map_default() {
    let mut some_map = test_map4();
    let runner = NodeRunner { tokens: vec!(NodeToken::SetDefault) };

    assert_eq!(4, some_map.len());
    some_map.node_step(runner);
    assert_eq!(0, some_map.len());
}

#[test]
fn map_get_keys() {
    let runner = NodeRunner { tokens: vec!(NodeToken::GetKeys) };
    assert_eq!("'bar', 'baz', 'foo', 'qux'", test_map4().node_step(runner));
}

#[test]
fn map_get() {
    let runner = NodeRunner { tokens: vec!(NodeToken::Get) };
    assert_eq!("{\n  \"foo\": 13\n}", test_map1().node_step(runner));
}

#[test]
fn map_set() {
    let mut some_map = test_map4();
    let runner = NodeRunner { tokens: vec!(NodeToken::Set(String::from("{\n  \"value\": 1,\n  \"string\": 99,\n  \"a somewhat unusual string\": 100\n}"))) };
    assert_eq!(some_map.node_step(runner), String::from(""));
    assert_eq!(3, some_map.len());
    assert_eq!(1,   *some_map.get("value").unwrap());
    assert_eq!(99,  *some_map.get("string").unwrap());
    assert_eq!(100, *some_map.get("a somewhat unusual string").unwrap());
}

#[test]
fn map_set_fail() {
    let mut some_map = test_map4();
    let runner = NodeRunner { tokens: vec!(NodeToken::Set(String::from("{\n  lol, 1]"))) };
    assert_eq!(some_map.node_step(runner), String::from("map set error: key must be a string at line 2 column 3"));
}

#[test]
fn map_help() {
    let output = r#"
Map Help

Commands:
*   help    - display this help
*   keys    - display the keys
*   get     - display JSON
*   set     - set to JSON
*   insert  - create a new element
*   remove  - remove an element
*   default - reset to empty map

Accessors:
*   [key]   - access item at the string key
*   .length - display number of items"#;
    let mut some_map = test_map4();
    let runner = NodeRunner { tokens: vec!(NodeToken::Help) };
    assert_eq!(some_map.node_step(runner), String::from(output));
}
