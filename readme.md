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

## Reuse

Some things cannot be changed to be configurable:

*   using a serde Serializer/Deserializer other then serde_json (would require generic closures)
*   removing the serde dependency entirely

This library is designed around the specific needs of a project of mine.
It is likely that you will need to either:

*   Make a fork and tweak it to your needs.
*   Take it as a proof of concept and build your own from scratch.
