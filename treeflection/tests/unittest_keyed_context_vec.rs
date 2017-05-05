extern crate treeflection;
#[macro_use] extern crate matches;
extern crate serde;

use treeflection::{Node, NodeRunner, NodeToken, KeyedContextVec};

fn test_vec4() -> KeyedContextVec<i32> {
    KeyedContextVec::from_vec(vec!((String::from("foo"), 100000), (String::from("bar"), 13), (String::from("baz"), -358), (String::from("qux"), 42)))
}

fn test_vec3() -> KeyedContextVec<i32> {
    KeyedContextVec::from_vec(vec!((String::from("foo"), 10), (String::from("bar"), 1337), (String::from("baz"), 42)))
}

fn test_vec2() -> KeyedContextVec<i32> {
    KeyedContextVec::from_vec(vec!((String::from("foo"), 10), (String::from("bar"), 1337)))
}

fn test_vec1() -> KeyedContextVec<i32> {
    KeyedContextVec::from_vec(vec!((String::from("foo"), 13)))
}

fn test_vec0() -> KeyedContextVec<i32> {
    KeyedContextVec::from_vec(vec!())
}

#[test]
fn selection_first() {
    let context_vec = KeyedContextVec::<i32>::new();
    assert!(matches!(context_vec.selection_first(), None));

    let mut context_vec = test_vec3();
    assert!(matches!(context_vec.selection_first(), None));

    context_vec.set_context(0);
    assert_eq!(*context_vec.selection_first().unwrap(), 10);

    context_vec.set_context(2);
    assert_eq!(*context_vec.selection_first().unwrap(), 42);

    context_vec.set_context_vec(vec!(0, 1, 2));
    assert_eq!(*context_vec.selection_first().unwrap(), 10);
}

#[test]
fn selection_first_mut() {
    // repeat immutable tests
    let mut context_vec = KeyedContextVec::<i32>::new();
    assert!(matches!(context_vec.selection_first_mut(), None));

    let mut context_vec = test_vec3();
    assert!(matches!(context_vec.selection_first_mut(), None));

    context_vec.set_context(0);
    assert_eq!(*context_vec.selection_first_mut().unwrap(), 10);

    context_vec.set_context(2);
    assert_eq!(*context_vec.selection_first_mut().unwrap(), 42);

    context_vec.set_context_vec(vec!(0, 1, 2));
    assert_eq!(*context_vec.selection_first_mut().unwrap(), 10);

    // test mutabality
    *context_vec.selection_first_mut().unwrap() = 9999;
    assert_eq!(context_vec[0], 9999);
    assert_eq!(context_vec[1], 1337);
    assert_eq!(context_vec[2], 42);
}

#[test]
fn selection() {
    let context_vec = KeyedContextVec::<i32>::new();
    assert_eq!(context_vec.selection().len(), 0);

    let mut context_vec = test_vec3();
    assert_eq!(context_vec.selection().len(), 0);

    context_vec.set_context(0);
    assert_eq!(context_vec.selection().len(), 1);
    assert_eq!(*context_vec.selection()[0], 10);

    context_vec.set_context(2);
    assert_eq!(context_vec.selection().len(), 1);
    assert_eq!(*context_vec.selection()[0], 42);

    context_vec.set_context_vec(vec!(0, 1, 2));
    assert_eq!(context_vec.selection().len(), 3);
    assert_eq!(*context_vec.selection()[0], 10);
    assert_eq!(*context_vec.selection()[1], 1337);
    assert_eq!(*context_vec.selection()[2], 42);
}

#[test]
fn clear_context() {
    let mut context_vec = test_vec3();
    context_vec.set_context(0);
    context_vec.clear_context();
    assert_eq!(context_vec.get_context().len(), 0);
}

#[test]
fn set_context() {
    let mut context_vec = test_vec3();

    context_vec.set_context(0);
    assert_eq!(context_vec.get_context().len(), 1);
    assert_eq!(context_vec.get_context()[0], 0);

    context_vec.set_context(1);
    assert_eq!(context_vec.get_context().len(), 1);
    assert_eq!(context_vec.get_context()[0], 1);

    context_vec.set_context(1);
    assert_eq!(context_vec.get_context().len(), 1);
    assert_eq!(context_vec.get_context()[0], 1);

    context_vec.set_context(2);
    assert_eq!(context_vec.get_context().len(), 1);
    assert_eq!(context_vec.get_context()[0], 2);
}

#[test]
#[should_panic]
fn set_context_out_of_bounds() {
    let mut context_vec = test_vec3();
    context_vec.set_context(3);
}

#[test]
fn set_context_vec() {
    let mut context_vec = test_vec3();

    context_vec.set_context_vec(vec!(1));
    assert_eq!(context_vec.get_context().len(), 1);
    assert_eq!(context_vec.get_context()[0], 1);

    context_vec.set_context_vec(vec!(1, 2));
    assert_eq!(context_vec.get_context().len(), 2);
    assert_eq!(context_vec.get_context()[0], 1);
    assert_eq!(context_vec.get_context()[1], 2);

    context_vec.set_context_vec(vec!(2, 1));
    assert_eq!(context_vec.get_context().len(), 2);
    assert_eq!(context_vec.get_context()[0], 2);
    assert_eq!(context_vec.get_context()[1], 1);
}

#[test]
fn set_vec() {
    let mut context_vec = test_vec3();
    context_vec.set_context(0);
    context_vec.set_vec(vec!((String::from("foo"), 1), (String::from("bar"), 99)));
    assert_eq!(context_vec.get_context().len(), 0);
    let vec = context_vec;
    assert_eq!(vec[0], 1);
    assert_eq!(vec[1], 99);
}

#[test]
fn clear() {
    let mut context_vec = test_vec3();
    context_vec.set_context(0);
    assert_eq!(context_vec.len(), 3);
    assert_eq!(context_vec.get_context().len(), 1);
    context_vec.clear();
    assert_eq!(context_vec.len(), 0);
    assert_eq!(context_vec.get_context().len(), 0);
}

#[test]
fn push() {
    let mut context_vec = test_vec3();
    assert_eq!(context_vec.len(), 3);
    context_vec.push(String::from("new"), 99);
    assert_eq!(context_vec.len(), 4);
    assert_eq!(context_vec[3], 99);
}

#[test]
fn insert() {
    let mut context_vec = test_vec3();
    context_vec.set_context_vec(vec!(0, 1, 2));

    assert_eq!(context_vec.get_context().len(), 3);
    assert_eq!(context_vec.len(), 3);
    context_vec.insert(3, String::from("new"), 99);
    assert_eq!(context_vec.get_context().len(), 3);
    assert_eq!(context_vec.get_context()[0], 0);
    assert_eq!(context_vec.get_context()[1], 1);
    assert_eq!(context_vec.get_context()[2], 2);
    assert_eq!(context_vec.len(), 4);
    assert_eq!(context_vec[0], 10);
    assert_eq!(context_vec[1], 1337);
    assert_eq!(context_vec[2], 42);
    assert_eq!(context_vec[3], 99);

    context_vec.insert(1, String::from("strings"), 101);
    assert_eq!(context_vec.get_context().len(), 3);
    assert_eq!(context_vec.get_context()[0], 0);
    assert_eq!(context_vec.get_context()[1], 2);
    assert_eq!(context_vec.get_context()[2], 3);
    assert_eq!(context_vec.len(), 5);
    assert_eq!(context_vec[0], 10);
    assert_eq!(context_vec[1], 101);
    assert_eq!(context_vec[2], 1337);
    assert_eq!(context_vec[3], 42);
    assert_eq!(context_vec[4], 99);
}

#[test]
fn pop() {
    let mut context_vec = KeyedContextVec::<bool>::new();
    assert!(matches!(context_vec.pop(), None));

    let mut context_vec = test_vec3();
    context_vec.set_context_vec(vec!(1, 2));

    assert_eq!(context_vec.get_context().len(), 2);
    assert_eq!(context_vec.len(), 3);
    assert!(matches!(context_vec.pop(), Some(42)));
    assert_eq!(context_vec.get_context().len(), 1);
    assert_eq!(context_vec.len(), 2);
}

#[test]
fn remove() {
    let mut context_vec = test_vec3();
    context_vec.remove(2);
    assert_eq!(context_vec.len(), 2);
    assert_eq!(context_vec[0], 10);
    assert_eq!(context_vec[1], 1337);
    assert_eq!(context_vec.get_context().len(), 0);

    let mut context_vec = test_vec3();
    context_vec.set_context(0);
    context_vec.remove(0);
    assert_eq!(context_vec.len(), 2);
    assert_eq!(context_vec[0], 1337);
    assert_eq!(context_vec[1], 42);
    let context = context_vec.get_context();
    assert_eq!(context.len(), 0);

    let mut context_vec = test_vec3();
    context_vec.set_context_vec(vec!(0, 1, 2));
    context_vec.remove(1);
    assert_eq!(context_vec.len(), 2);
    assert_eq!(context_vec[0], 10);
    assert_eq!(context_vec[1], 42);
    let context = context_vec.get_context();
    assert_eq!(context.len(), 2);
    assert_eq!(context[0], 0);
    assert_eq!(context[1], 1);
}

#[test]
#[should_panic]
fn remove_out_of_bounds() {
    let mut context_vec = test_vec3();
    context_vec.remove(3);
}

#[test]
fn key_to_value() {
    let some_vec = test_vec4();
    assert_eq!(*some_vec.key_to_value("foo").unwrap(), 100000);
    assert_eq!(*some_vec.key_to_value("bar").unwrap(), 13);
    assert_eq!(*some_vec.key_to_value("qux").unwrap(), 42);
    assert_eq!(*some_vec.key_to_value("baz").unwrap(), -358);
    assert!(some_vec.key_to_value("none").is_none());
}

#[test]
fn index_to_key() {
    let some_vec = test_vec4();
    assert_eq!(*some_vec.index_to_key(0).unwrap(), String::from("foo"));
    assert_eq!(*some_vec.index_to_key(1).unwrap(), String::from("bar"));
    assert_eq!(*some_vec.index_to_key(3).unwrap(), String::from("qux"));
    assert_eq!(*some_vec.index_to_key(2).unwrap(), String::from("baz"));
    assert!(some_vec.index_to_key(9).is_none());
}

#[test]
fn key_to_index() {
    let some_vec = test_vec4();
    assert_eq!(some_vec.key_to_index("foo").unwrap(), 0);
    assert_eq!(some_vec.key_to_index("bar").unwrap(), 1);
    assert_eq!(some_vec.key_to_index("qux").unwrap(), 3);
    assert_eq!(some_vec.key_to_index("baz").unwrap(), 2);
    assert!(some_vec.key_to_index("none").is_none());
}

#[test]
fn key_value_iter() {
    let some_vec = test_vec2();
    let mut iter = some_vec.key_value_iter();
    if let Some((key, value)) = iter.next() {
        assert_eq!(*key, String::from("foo"));
        assert_eq!(*value, 10);
    }
    else {
        panic!();
    }

    if let Some((key, value)) = iter.next() {
        assert_eq!(*key, String::from("bar"));
        assert_eq!(*value, 1337);
    }
    else {
        panic!();
    }

    assert!(iter.next().is_none());
}

#[test]
fn deref_coercion() {
    let context_vec = test_vec3();
    assert_eq!(context_vec[0], 10);
    assert_eq!(context_vec[1], 1337);
    assert_eq!(context_vec[2], 42);
    assert_eq!(*context_vec.first().unwrap(), 10);
    assert_eq!(context_vec.len(), 3);
    assert_eq!(context_vec.is_empty(), false);
    assert!(context_vec.contains(&1337));

    let mut iter = context_vec.iter();
    assert_eq!(*iter.next().unwrap(), 10);
    assert_eq!(*iter.next().unwrap(), 1337);
    assert_eq!(*iter.next().unwrap(), 42);
    assert!(iter.next().is_none());
}

#[test]
fn deref_mut_coercion() {
    let mut context_vec = test_vec3();
    if let Some(x) = context_vec.first_mut() {
        *x = 4;
    }
    assert_eq!(*context_vec.first().unwrap(), 4);
}

#[test]
fn index() {
    let some_vec = test_vec4();
    assert_eq!(some_vec[0], 100000);
    assert_eq!(some_vec[0..].len(), 4);
    assert_eq!(some_vec[0..4].len(), 4);
    assert_eq!(some_vec[..4].len(), 4);
    assert_eq!(some_vec[..].len(), 4);

    assert_eq!(some_vec["foo"], 100000);
    assert_eq!(some_vec["bar"], 13);
}

#[test]
fn index_mut() {
    let mut some_vec = test_vec4();

    some_vec[0] = 413;
    assert_eq!(some_vec[0], 413);

    some_vec["foo"] = 700;
    assert_eq!(some_vec["foo"], 700);
}

#[test]
#[should_panic]
fn index_out_of_bounds() {
    let context_vec = test_vec3();
    context_vec[3];
}

#[test]
#[should_panic]
fn key_unused() {
    let context_vec = test_vec3();
    context_vec["glub"];
}

/*
 * node_step() tests
 */

#[test]
fn help() {
    let mut context_vec = test_vec3();
    let runner = NodeRunner { tokens: vec!(NodeToken::Help) };
    let output = r#"
Keyed Context Vector Help

Commands:
*   help               - display this help
*   get                - display JSON
*   set                - set to JSON
*   insert $KEY        - create a new element at the end of the vector with $KEY
*   insert $INDEX $KEY - create a new element at $INDEX with $KEY
*   remove             - remove the element
*   remove $KEY        - remove the element with $KEY
*   remove $INDEX      - remove the element at $INDEX
*   default            - reset to default values

Accessors:
*   [INDEX] - access item at INDEX
*   [?]     - access items at current context
*   .length - display number of items"#;
    assert_eq!(context_vec.node_step(runner), String::from(output));
}

#[test]
fn node_step_chain_context() {
    let mut context_vec = test_vec3();
    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainContext
    )};

    assert_eq!("|", context_vec.node_step(runner.clone()).as_str());

    context_vec.set_context(0);
    assert_eq!("|10|", context_vec.node_step(runner.clone()).as_str());

    context_vec.set_context(1);
    assert_eq!("|1337|", context_vec.node_step(runner.clone()).as_str());

    context_vec.set_context(2);
    assert_eq!("|42|", context_vec.node_step(runner.clone()).as_str());

    context_vec.set_context_vec(vec!(2, 0));
    assert_eq!("|42|10|", context_vec.node_step(runner.clone()).as_str());

    context_vec.set_context_vec(vec!(0, 1, 2));
    assert_eq!("|10|1337|42|", context_vec.node_step(runner.clone()).as_str());
}

#[test]
fn node_step_chain_index() {
    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainIndex(0),
    )};
    assert_eq!("100000", test_vec4().node_step(runner));

    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainIndex(1),
    )};
    assert_eq!("13", test_vec4().node_step(runner));

    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainIndex(2),
    )};
    assert_eq!("-358", test_vec4().node_step(runner));

    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainIndex(3),
    )};
    assert_eq!("42", test_vec4().node_step(runner));

    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainIndex(4),
    )};
    assert_eq!(test_vec4().node_step(runner), "Used index 4 on a keyed context vector of size 4 (try a value between 0-3)");

    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainIndex(1),
    )};
    assert_eq!(test_vec1().node_step(runner), "Used index 1 on a keyed context vector of size 1 (try 0)");

    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainIndex(0),
    )};
    assert_eq!(test_vec0().node_step(runner), "Used index 0 on an empty keyed context vector");
}

#[test]
fn nodestep_chain_key() {
    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainKey(String::from("foo")),
    )};
    assert_eq!("100000", test_vec4().node_step(runner));

    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainKey(String::from("bar")),
    )};
    assert_eq!("13", test_vec4().node_step(runner));

    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainKey(String::from("baz")),
    )};
    assert_eq!("-358", test_vec4().node_step(runner));

    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainKey(String::from("qux")),
    )};
    assert_eq!("42", test_vec4().node_step(runner));

    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainKey(String::from("quux")),
    )};
    assert_eq!(test_vec4().node_step(runner), "Used key 'quux' on a keyed context vector that does not contain it. Try one of: 'foo', 'bar', 'baz', 'qux'");

    let runner = NodeRunner { tokens: vec!(
        NodeToken::Get,
        NodeToken::ChainKey(String::from("map")),
    )};
    assert_eq!(test_vec0().node_step(runner), "Used key 'map' on an empty keyed context vector.");
}

#[test]
fn node_step_insert_key() {
    let mut some_vec = test_vec4();

    assert_eq!(some_vec.len(), 4);
    assert_eq!(some_vec[0], 100000);
    assert_eq!(some_vec[1], 13);
    assert_eq!(some_vec[2], -358);
    assert_eq!(some_vec[3], 42);

    let runner = NodeRunner { tokens: vec!(NodeToken::InsertKey(String::from("foo"))) };
    assert_eq!(some_vec.node_step(runner), String::from("Tried to insert with key 'foo' on a keyed context vector that already contains it. Current keys: 'foo', 'bar', 'baz', 'qux'"));
    assert_eq!(some_vec.len(), 4);
    assert_eq!(some_vec[0], 100000);
    assert_eq!(some_vec[1], 13);
    assert_eq!(some_vec[2], -358);
    assert_eq!(some_vec[3], 42);

    let runner = NodeRunner { tokens: vec!(NodeToken::InsertKey(String::from("new"))) };
    assert_eq!("", some_vec.node_step(runner));
    assert_eq!(some_vec.len(), 5);
    assert_eq!(some_vec[0], 100000);
    assert_eq!(some_vec[1], 13);
    assert_eq!(some_vec[2], -358);
    assert_eq!(some_vec[3], 42);
    assert_eq!(some_vec[4], 0);

    let runner = NodeRunner { tokens: vec!(NodeToken::InsertKey(String::from("string"))) };
    assert_eq!("", some_vec.node_step(runner));
    assert_eq!(some_vec.len(), 6);
    assert_eq!(some_vec[0], 100000);
    assert_eq!(some_vec[1], 13);
    assert_eq!(some_vec[2], -358);
    assert_eq!(some_vec[3], 42);
    assert_eq!(some_vec[4], 0);
    assert_eq!(some_vec[5], 0);
}

#[test]
fn node_step_insert_index_key() {
    let mut some_vec = test_vec4();

    assert_eq!(some_vec.len(), 4);
    assert_eq!(some_vec[0], 100000);
    assert_eq!(some_vec[1], 13);
    assert_eq!(some_vec[2], -358);
    assert_eq!(some_vec[3], 42);

    let runner = NodeRunner { tokens: vec!(NodeToken::InsertIndexKey(2, String::from("foo"))) };
    assert_eq!(some_vec.node_step(runner), String::from("Tried to insert with key 'foo' on a keyed context vector that already contains it. Current keys: 'foo', 'bar', 'baz', 'qux'"));
    assert_eq!(some_vec.len(), 4);
    assert_eq!(some_vec[0], 100000);
    assert_eq!(some_vec[1], 13);
    assert_eq!(some_vec[2], -358);
    assert_eq!(some_vec[3], 42);

    let runner = NodeRunner { tokens: vec!(NodeToken::InsertIndexKey(5, String::from("foo"))) };
    assert_eq!(some_vec.node_step(runner), String::from("Tried to insert at index 5 on a keyed context vector of size 4 (try a value between 0-4)"));
    assert_eq!(some_vec.len(), 4);
    assert_eq!(some_vec[0], 100000);
    assert_eq!(some_vec[1], 13);
    assert_eq!(some_vec[2], -358);
    assert_eq!(some_vec[3], 42);

    let runner = NodeRunner { tokens: vec!(NodeToken::InsertIndexKey(5, String::from("new"))) };
    assert_eq!(some_vec.node_step(runner), String::from("Tried to insert at index 5 on a keyed context vector of size 4 (try a value between 0-4)"));
    assert_eq!(some_vec.len(), 4);
    assert_eq!(some_vec[0], 100000);
    assert_eq!(some_vec[1], 13);
    assert_eq!(some_vec[2], -358);
    assert_eq!(some_vec[3], 42);

    let runner = NodeRunner { tokens: vec!(NodeToken::InsertIndexKey(0, String::from("new"))) };
    assert_eq!("", some_vec.node_step(runner));
    assert_eq!(some_vec.len(), 5);
    assert_eq!(some_vec[0], 0);
    assert_eq!(some_vec[1], 100000);
    assert_eq!(some_vec[2], 13);
    assert_eq!(some_vec[3], -358);
    assert_eq!(some_vec[4], 42);

    let runner = NodeRunner { tokens: vec!(NodeToken::InsertIndexKey(2, String::from("string"))) };
    assert_eq!("", some_vec.node_step(runner));
    assert_eq!(some_vec.len(), 6);
    assert_eq!(some_vec[0], 0);
    assert_eq!(some_vec[1], 100000);
    assert_eq!(some_vec[2], 0);
    assert_eq!(some_vec[3], 13);
    assert_eq!(some_vec[4], -358);
    assert_eq!(some_vec[5], 42);
}

#[test]
fn node_step_remove() {
    let mut some_vec = test_vec2();

    assert_eq!(some_vec.len(), 2);
    assert_eq!(some_vec[0], 10);
    assert_eq!(some_vec[1], 1337);

    let runner = NodeRunner { tokens: vec!(NodeToken::Remove) };
    assert_eq!("", some_vec.node_step(runner));
    assert_eq!(some_vec.len(), 1);
    assert_eq!(some_vec[0], 10);

    let runner = NodeRunner { tokens: vec!(NodeToken::Remove) };
    assert_eq!("", some_vec.node_step(runner));
    assert_eq!(some_vec.len(), 0);

    let runner = NodeRunner { tokens: vec!(NodeToken::Remove) };
    assert_eq!("Tried to remove from an empty keyed context vector.", some_vec.node_step(runner));
    assert_eq!(some_vec.len(), 0);
}

#[test]
fn node_step_remove_index() {
    let mut some_vec = test_vec4();

    assert_eq!(some_vec.len(), 4);
    assert_eq!(some_vec[0], 100000);
    assert_eq!(some_vec[1], 13);
    assert_eq!(some_vec[2], -358);
    assert_eq!(some_vec[3], 42);

    let runner = NodeRunner { tokens: vec!(NodeToken::RemoveIndex(4)) };
    assert_eq!(some_vec.node_step(runner), "Tried to remove the value at index 4 on a keyed context vector of size 4 (try a value between 0-3)");
    assert_eq!(some_vec.len(), 4);
    assert_eq!(some_vec[0], 100000);
    assert_eq!(some_vec[1], 13);
    assert_eq!(some_vec[2], -358);
    assert_eq!(some_vec[3], 42);

    let runner = NodeRunner { tokens: vec!(NodeToken::RemoveIndex(0)) };
    assert_eq!("", some_vec.node_step(runner));
    assert_eq!(some_vec.len(), 3);
    assert_eq!(some_vec[0], 13);
    assert_eq!(some_vec[1], -358);
    assert_eq!(some_vec[2], 42);

    let runner = NodeRunner { tokens: vec!(NodeToken::RemoveIndex(2)) };
    assert_eq!("", some_vec.node_step(runner));
    assert_eq!(some_vec.len(), 2);
    assert_eq!(some_vec[0], 13);
    assert_eq!(some_vec[1], -358);
}

#[test]
fn node_step_remove_key() {
    let mut some_vec = test_vec4();

    assert_eq!(some_vec.len(), 4);
    assert_eq!(some_vec[0], 100000);
    assert_eq!(some_vec[1], 13);
    assert_eq!(some_vec[2], -358);
    assert_eq!(some_vec[3], 42);

    let runner = NodeRunner { tokens: vec!(NodeToken::RemoveKey(String::from("boo"))) };
    assert_eq!(some_vec.node_step(runner), "Tried to remove the value with key 'boo' on a keyed context vector that doesnt contain it. Current keys: 'foo', 'bar', 'baz', 'qux'");
    assert_eq!(some_vec.len(), 4);
    assert_eq!(some_vec[0], 100000);
    assert_eq!(some_vec[1], 13);
    assert_eq!(some_vec[2], -358);
    assert_eq!(some_vec[3], 42);

    let runner = NodeRunner { tokens: vec!(NodeToken::RemoveKey(String::from("foo"))) };
    assert_eq!("", some_vec.node_step(runner));
    assert_eq!(some_vec.len(), 3);
    assert_eq!(some_vec[0], 13);
    assert_eq!(some_vec[1], -358);
    assert_eq!(some_vec[2], 42);

    let runner = NodeRunner { tokens: vec!(NodeToken::RemoveKey(String::from("baz"))) };
    assert_eq!("", some_vec.node_step(runner));
    assert_eq!(some_vec.len(), 2);
    assert_eq!(some_vec[0], 13);
    assert_eq!(some_vec[1], -358);
}

#[test]
fn node_step_reset() {
    let mut some_vec = test_vec4();
    let runner = NodeRunner { tokens: vec!(NodeToken::SetDefault) };

    assert_eq!(4, some_vec.len());
    some_vec.node_step(runner);
    assert_eq!(0, some_vec.len());
}

#[test]
fn node_step_get() {
    let runner = NodeRunner { tokens: vec!(NodeToken::Get) };
    assert_eq!("[\n  100000,\n  13,\n  -358,\n  42\n]", test_vec4().node_step(runner));
}

#[test]
fn node_step_set() {
    let mut some_vec = test_vec4();
    let runner = NodeRunner { tokens: vec!(NodeToken::Set(String::from("[1, 2, 99, 100]"))) };
    assert_eq!(some_vec.node_step(runner), String::from(""));
    assert_eq!(1, some_vec[0]);
    assert_eq!(2, some_vec[1]);
    assert_eq!(99, some_vec[2]);
    assert_eq!(100, some_vec[3]);
}

#[test]
fn node_step_set_fail()
{
    let mut some_vec = test_vec4();
    let runner = NodeRunner { tokens: vec!(NodeToken::Set(String::from("[1, lol]"))) };
    assert_eq!(some_vec.node_step(runner), String::from("keyed context vector set error: expected value at line 1 column 5"));
}
