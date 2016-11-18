#[macro_use] extern crate treeflection;
#[macro_use] extern crate matches;
extern crate serde;

use treeflection::{Node, NodeRunner, NodeToken, ContextVec};

fn test_vec() -> ContextVec<i32> {
    ContextVec::from_vec(vec!(10, 1337, 42))
}

#[test]
fn test_chain_context() {
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
fn test_selection_first() {
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
fn test_selection() {
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
fn test_clear_context() {
    let mut context_vec = test_vec();
    context_vec.set_context(0);
    context_vec.clear_context();
    assert_eq!(context_vec.get_context().len(), 0);
}

#[test]
fn test_set_context() {
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
fn test_set_context_out_of_bounds() {
    let mut context_vec = test_vec();
    context_vec.set_context(3);
}

#[test]
fn test_set_context_vec() {
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
fn test_set_vec() {
    let mut context_vec = test_vec();
    context_vec.set_context(0);
    context_vec.set_vec(vec!(1, 99));
    assert_eq!(context_vec.get_context().len(), 0);
    let vec = context_vec;
    assert_eq!(vec[0], 1);
    assert_eq!(vec[1], 99);
}

#[test]
fn test_clear() {
    let mut context_vec = test_vec();
    context_vec.set_context(0);
    assert_eq!(context_vec.len(), 3);
    assert_eq!(context_vec.get_context().len(), 1);
    context_vec.clear();
    assert_eq!(context_vec.len(), 0);
    assert_eq!(context_vec.get_context().len(), 0);
}

#[test]
fn test_push() {
    let mut context_vec = test_vec();
    assert_eq!(context_vec.len(), 3);
    context_vec.push(99);
    assert_eq!(context_vec.len(), 4);
    assert_eq!(context_vec[3], 99);
}

#[test]
fn test_insert() {
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
fn test_pop() {
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
fn test_remove() {
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
fn test_remove_out_of_bounds() {
    let mut context_vec = test_vec();
    context_vec.remove(3);
}

#[test]
fn test_deref_coercion() {
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
fn test_deref_mut_coercion() {
    let mut context_vec = test_vec();
    if let Some(x) = context_vec.first_mut() {
        *x = 4;
    }
    assert_eq!(*context_vec.first().unwrap(), 4);
}

#[test]
#[should_panic]
fn test_index_out_of_bounds() {
    let context_vec = test_vec();
    context_vec[3];
}
