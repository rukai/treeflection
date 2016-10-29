# Treeflection Command Manual

This manual does not accurately reflect current functionality.

## Syntax

TODO: Exact description

## Actions

You can implement your own actions for your own types.

Standard types have the following actions implemented by default:
*   <attribute>.set <value> - change an attribute to the specified size
*   <attribute>.get <depth> - display an attribute, the depth argument is optional and specifies how deeply nested object attributes should be shown.
*   <attribute>.copy        - copy the specified attribute
*   <attribute>.paste       - paste the copied attribute to the specified attribute (Must be the same type)

## Indexing

By default indexing is implemented for Vec and HashMap:

*   `foo["M"]`             HashMaps can be accessed via strings
*   `bar[0]`               select element 0 of bar
*   `bar[0, 1-5]`          select element 0 and elements between 1 and 5 inclusive
*   `bar[2-4].fighters[*]` select all fighters in packages 2, 3 and 4
*   `bar[*]`               select all packages
*   `bar[?]`               select based on [context](link_to_context_section)
*   `bar[?+1]`             select the element after the current context
*   `bar[2, ?-1]`          select element 2 and the element before the current context
