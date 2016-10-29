# Treeflection

Treeflection runs a command stored as a string on a tree of structs & primitive types.

## Commands

A command to set an int in a Vec in a struct in another struct in a Hashmap to the value 50 looks like:
`someHashMap["key"].someChild.anotherChild[0] set 50`

For the full syntax take a look at the [Command Manual](commandManual.md)

## Usage

The `Node` trait must be implemented for every type in the tree.
Then a new `NodeRunner` is created using the command string and passed to the node_step method of the root node.
The `NodeRunner` is then passed to the children specified in the command and then runs the command on the final specified child.
Use the treeflection_derive crate to #[Derive(Node)] for your own structs or write your own handlers.

TODO: give proper example
