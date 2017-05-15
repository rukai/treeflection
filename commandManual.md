# Treeflection Command Manual

## Syntax

Must have an action.
May optionally be preceded by any number of accessors

## Access property

Dot notation is used to access the properties of structs and enum structs:

`foo.bar.baz`

## Access value by index/key

Index notation is used to access the properties of Vec, HashMap, tuples and tuple enums

*   `property["key_string"]` select by key
*   `property[0]`            select by index
*   `property[?]`            select based on context
*   `property[*]`            select all

## Actions

Each struct/primitive has its own set of actions available.
However there are some actions that you will find on every type:
*   `:help`        - Use this to find properties and actions a node has.
*   `:set <value>` - set the property to the specified json input
*   `:get`         - display the attribute in json

## Context

The ContextVec struct (and other Context* structs) allow you to set indexes as the context.
When the context accessor '[?]' is used on that struct it accesses the current context.

## Examples

A super simple command to get help for the root node.
`:help`

A command to set an int in a Vec in a struct in another struct in a Hashmap to the value 50 looks like:
`someHashMap["key"].someChild.anotherChild[0]:set 50`

