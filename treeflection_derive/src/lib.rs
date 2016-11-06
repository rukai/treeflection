#![feature(proc_macro, proc_macro_lib)]

extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(Node)]
pub fn my_macro(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    format!("{}\n impl Node for Foo {{ fn node_step(&mut self, runner: NodeRunner) -> String {{ String::from(\"lel\") }} }}", input).parse().unwrap()
}
