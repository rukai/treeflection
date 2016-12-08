#[macro_use] extern crate treeflection;
#[macro_use] extern crate matches;
extern crate serde;

use treeflection::{Node, NodeRunner, NodeToken, ContextVec};

fn test_vec() -> ContextVec<i32> {
    ContextVec::from_vec(vec!(10, 1337, 42))
}

fn test_vec4() -> ContextVec<i32> {
    ContextVec::from_vec(vec!(100000, 13, -358, 42))
}

#[test]
fn selection_first() {
    let context_vec = ContextVec::<i32>::new();
    assert!(matches!(context_vec.selection_first(), None));

    let mut context_vec = test_vec();
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
    let mut context_vec = ContextVec::<i32>::new();
    assert!(matches!(context_vec.selection_first_mut(), None));

    let mut context_vec = test_vec();
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
    let context_vec = ContextVec::<i32>::new();
    assert_eq!(context_vec.selection().len(), 0);

    let mut context_vec = test_vec();
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
    let mut context_vec = test_vec();
    context_vec.set_context(0);
    context_vec.clear_context();
    assert_eq!(context_vec.get_context().len(), 0);
}

#[test]
fn set_context() {
    let mut context_vec = test_vec();

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
    let mut context_vec = test_vec();
    context_vec.set_context(3);
}

#[test]
fn set_context_vec() {
    let mut context_vec = test_vec();

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
    let mut context_vec = test_vec();
    context_vec.set_context(0);
    context_vec.set_vec(vec!(1, 99));
    assert_eq!(context_vec.get_context().len(), 0);
    let vec = context_vec;
    assert_eq!(vec[0], 1);
    assert_eq!(vec[1], 99);
}

#[test]
fn clear() {
    let mut context_vec = test_vec();
    context_vec.set_context(0);
    assert_eq!(context_vec.len(), 3);
    assert_eq!(context_vec.get_context().len(), 1);
    context_vec.clear();
    assert_eq!(context_vec.len(), 0);
    assert_eq!(context_vec.get_context().len(), 0);
}

#[test]
fn push() {
    let mut context_vec = test_vec();
    assert_eq!(context_vec.len(), 3);
    context_vec.push(99);
    assert_eq!(context_vec.len(), 4);
    assert_eq!(context_vec[3], 99);
}

#[test]
fn insert() {
    let mut context_vec = test_vec();
    context_vec.set_context_vec(vec!(0, 1, 2));

    assert_eq!(context_vec.get_context().len(), 3);
    assert_eq!(context_vec.len(), 3);
    context_vec.insert(3, 99);
    assert_eq!(context_vec.get_context().len(), 3);
    assert_eq!(context_vec.get_context()[0], 0);
    assert_eq!(context_vec.get_context()[1], 1);
    assert_eq!(context_vec.get_context()[2], 2);
    assert_eq!(context_vec.len(), 4);
    assert_eq!(context_vec[0], 10);
    assert_eq!(context_vec[1], 1337);
    assert_eq!(context_vec[2], 42);
    assert_eq!(context_vec[3], 99);

    context_vec.insert(1, 101);
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
    let mut context_vec = ContextVec::<bool>::new();
    assert!(matches!(context_vec.pop(), None));

    let mut context_vec = test_vec();
    context_vec.set_context_vec(vec!(1, 2));

    assert_eq!(context_vec.get_context().len(), 2);
    assert_eq!(context_vec.len(), 3);
    assert!(matches!(context_vec.pop(), Some(42)));
    assert_eq!(context_vec.get_context().len(), 1);
    assert_eq!(context_vec.len(), 2);
}

#[test]
fn remove() {
    let mut context_vec = test_vec();
    context_vec.remove(2);
    assert_eq!(context_vec.len(), 2);
    assert_eq!(context_vec[0], 10);
    assert_eq!(context_vec[1], 1337);
    assert_eq!(context_vec.get_context().len(), 0);

    let mut context_vec = test_vec();
    context_vec.set_context(0);
    context_vec.remove(0);
    assert_eq!(context_vec.len(), 2);
    assert_eq!(context_vec[0], 1337);
    assert_eq!(context_vec[1], 42);
    let context = context_vec.get_context();
    assert_eq!(context.len(), 0);

    let mut context_vec = test_vec();
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
    let mut context_vec = test_vec();
    context_vec.remove(3);
}

#[test]
fn deref_coercion() {
    let context_vec = test_vec();
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
    let mut context_vec = test_vec();
    if let Some(x) = context_vec.first_mut() {
        *x = 4;
    }
    assert_eq!(*context_vec.first().unwrap(), 4);
}

#[test]
#[should_panic]
fn index_out_of_bounds() {
    let context_vec = test_vec();
    context_vec[3];
}

// node_step tests
// tests beggining with vec_ are the same as the tests used for impl Node for Vec

#[test]
fn chain_context() {
    let mut context_vec = test_vec();
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
fn help() {
    let mut context_vec = test_vec();
    let runner = NodeRunner { tokens: vec!(NodeToken::Help) };
    let output = r#"
Context Vector Help

Commands:
*   help    - display this help
*   get     - display JSON
*   set     - set to JSON
*   insert  - create a new element
*   remove  - remove an element
*   default - reset to default values

Accessors:
*   [index] - access item at index
*   [?]     - access items at current context
*   .length - display number of items"#;
    assert_eq!(context_vec.node_step(runner), String::from(output));
}

#[test]
fn vec_insert() {
    let mut some_vec = test_vec4();

    assert_eq!(some_vec.len(), 4);
    assert_eq!(some_vec[0], 100000);
    assert_eq!(some_vec[1], 13);
    assert_eq!(some_vec[2], -358);
    assert_eq!(some_vec[3], 42);

    let runner = NodeRunner { tokens: vec!(NodeToken::Insert(0)) };
    assert_eq!("", some_vec.node_step(runner));
    assert_eq!(some_vec.len(), 5);
    assert_eq!(some_vec[0], 0);
    assert_eq!(some_vec[1], 100000);
    assert_eq!(some_vec[2], 13);
    assert_eq!(some_vec[3], -358);
    assert_eq!(some_vec[4], 42);

    let runner = NodeRunner { tokens: vec!(NodeToken::Insert(2)) };
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
fn vec_remove() {
    let mut some_vec = test_vec4();

    assert_eq!(some_vec.len(), 4);
    assert_eq!(some_vec[0], 100000);
    assert_eq!(some_vec[1], 13);
    assert_eq!(some_vec[2], -358);
    assert_eq!(some_vec[3], 42);

    let runner = NodeRunner { tokens: vec!(NodeToken::Remove(0)) };
    assert_eq!("", some_vec.node_step(runner));
    assert_eq!(some_vec.len(), 3);
    assert_eq!(some_vec[0], 13);
    assert_eq!(some_vec[1], -358);
    assert_eq!(some_vec[2], 42);

    let runner = NodeRunner { tokens: vec!(NodeToken::Remove(2)) };
    assert_eq!("", some_vec.node_step(runner));
    assert_eq!(some_vec.len(), 2);
    assert_eq!(some_vec[0], 13);
    assert_eq!(some_vec[1], -358);
}

#[test]
fn vec_reset() {
    let mut some_vec = test_vec4();
    let runner = NodeRunner { tokens: vec!(NodeToken::SetDefault) };

    assert_eq!(4, some_vec.len());
    some_vec.node_step(runner);
    assert_eq!(0, some_vec.len());
}

#[test]
fn vec_get() {
    let runner = NodeRunner { tokens: vec!(NodeToken::Get) };
    assert_eq!("[\n  100000,\n  13,\n  -358,\n  42\n]", test_vec4().node_step(runner));
}

#[test]
fn vec_set() {
    let mut some_vec = test_vec4();
    let runner = NodeRunner { tokens: vec!(NodeToken::Set(String::from("[1, 2, 99, 100]"))) };
    assert_eq!(some_vec.node_step(runner), String::from(""));
    assert_eq!(1, some_vec[0]);
    assert_eq!(2, some_vec[1]);
    assert_eq!(99, some_vec[2]);
    assert_eq!(100, some_vec[3]);
}

#[test]
fn vec_set_fail()
{
    let mut some_vec = test_vec4();
    let runner = NodeRunner { tokens: vec!(NodeToken::Set(String::from("[1, lol]"))) };
    assert_eq!(some_vec.node_step(runner), String::from("vector set error: expected value at line 1 column 5"));
}
