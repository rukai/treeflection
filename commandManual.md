# Treeflection Command Manual

This manual does not accurately reflect current functionality.

Your own types can impl the actions your own way or you can use the treeflection_derive crate which will automatically impl as below.

## Syntax

TODO: Exact description

## Actions

Standard types have the following actions implemented by default:
*   `<property> set <value>` - set the property to the specified json input
*   `<property> get`         - display the attribute in json
*   `<property> copy`        - stores the result of `get` in your clipboard
*   `<property> paste`       - runs the contents of your clipboard on `set`

## Dot Notation

Dot notation is used to access the properties of structs and enum structs:

`foo.bar.baz`

## Index Notation

Index notation is used to access the properties of Vec, HashMap, tuples and tuple enums

*   `foo["M"]`             HashMaps can be accessed via strings
*   `bar[0]`               select element 0 of bar
*   `bar[0, 1-5]`          select element 0 and elements between 1 and 5 inclusive
*   `bar[2-4].fighters[*]` select all fighters in packages 2, 3 and 4
*   `bar[*]`               select all packages
*   `bar[?]`               select based on [context](link_to_context_section)
*   `bar[?+1]`             select the element after the current context
*   `bar[2, ?-1]`          select element 2 and the element before the current context
