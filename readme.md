# Treeflection [![Build Status](https://travis-ci.org/rukai/treeflection.svg?branch=master)](https://travis-ci.org/rukai/treeflection) [![dependency status](https://deps.rs/repo/github/rukai/treeflection/status.svg)](https://deps.rs/repo/github/rukai/treeflection)

Treeflection runs a command stored as a string on a tree of structs, collections and primitive types.

## Commands

A command to set an int in a Vec in a struct in another struct in a Hashmap to the value 50 looks like:
`someHashMap["key"].someChild.anotherChild[0]:set 50`

For the full syntax take a look at the [Command Manual](commandManual.md)

## Usage

The `Node` trait must be implemented for every type in the tree.
Then a new `NodeRunner` is created using the command string and passed to the node_step method of the root node.
The `NodeRunner` is then passed to the children specified in the command and then runs the command on the final specified child.
Use the treeflection_derive crate to #[Derive(Node)] your own structs or write your own handlers.

### Vec example

```rust
extern crate treeflection;
use treeflection::{NodeRunner, Node};

pub fn main() {
    let mut test_vec = vec!(0, 413, 358, 42);

    let command = "[1]:get";
    let result = test_vec.node_step(NodeRunner::new(command).unwrap());
    assert_eq!(result, "413");

    let command = "[1]:set 1111";
    let result = test_vec.node_step(NodeRunner::new(command).unwrap());
    assert_eq!(result, "");

    let command = "[1]:get";
    let result = test_vec.node_step(NodeRunner::new(command).unwrap());
    assert_eq!(result, "1111");
}
```

### Custom struct example

Use the treeflection_derive crate to #[Derive(Node)] your own structs or write your own handlers.
Your structs also need to impl the traits Serialize, Deserialize, Default and Clone as well as `extern crate serde_json`.
This is because serde_json is used to get/set entire structs.

Currently `use treeflection::{NodeRunner, Node, NodeToken};` must be included so the macro can access these types.

```rust
extern crate treeflection;
#[macro_use] extern crate treeflection_derive;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

use treeflection::{NodeRunner, Node, NodeToken};

#[derive(Node, Serialize, Deserialize, Default, Clone)]
struct SolarSystem {
    pub mercury:  Planet,
    pub earth:    Planet,
    pub mars:     Planet,
        planet_x: Planet,
}

impl SolarSystem {
    pub fn new() -> SolarSystem {
        SolarSystem {
            mercury:  Planet { radius: 2440.0 },
            earth:    Planet { radius: 6371.0 },
            mars:     Planet { radius: 3390.0 },
            planet_x: Planet { radius: 1337.0 },
        }
    }
}

#[NodeActions(
    // we want the function circumference to be accessible via treeflection by the same name
    NodeAction(function="circumference", return_string),

    // we want the function explode_internal_naming_scheme to be accessible via treeflection
    // by the name explode and we want to ignore its return value so that it will compile despite not returning a String
    NodeAction(action="explode", function="explode_internal_naming_scheme"),
)]
#[derive(Node, Serialize, Deserialize, Default, Clone)]
struct Planet {
    pub radius: f32
}

impl Planet {
    pub fn circumference(&self) -> String {
        (self.radius * 2.0 * std::f32::consts::PI).to_string()
    }

    pub fn explode_internal_naming_scheme(&mut self) {
        self.radius = 0.0;
    }
}


pub fn main() {
    let mut ss = SolarSystem::new();

    // serialize the whole struct into json
    let command = ":get";
    let result = ss.node_step(NodeRunner::new(command).unwrap());
    assert_eq!(result,
r#"{
  "mercury": {
    "radius": 2440.0
  },
  "earth": {
    "radius": 6371.0
  },
  "mars": {
    "radius": 3390.0
  },
  "planet_x": {
    "radius": 1337.0
  }
}"#);

    // access properties
    let command = "earth.radius:get";
    let result = ss.node_step(NodeRunner::new(command).unwrap());
    assert_eq!(result, "6371");

    // call methods on the struct
    let command = "earth:circumference";
    let result = ss.node_step(NodeRunner::new(command).unwrap());
    assert_eq!(result, "40030.176");

    let command = "earth:explode";
    let result = ss.node_step(NodeRunner::new(command).unwrap());
    assert_eq!(result, "");
    assert_eq!(ss.earth.radius, 0.0);

    // private properties are not accessible via treeflection
    let command = "planet_x:get";
    let result = ss.node_step(NodeRunner::new(command).unwrap());
    assert_eq!(result, "SolarSystem does not have a property 'planet_x'");
}
```

## Contributing

This library is designed around the specific needs of [PF Sandbox](https://github.com/rukai/PF_Sandbox).
Pull requests are welcome but if the changes go against the needs of PF Sandbox you will be stuck with your own fork. :)
