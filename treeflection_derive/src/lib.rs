#![feature(proc_macro, proc_macro_lib)]

extern crate syn;
#[macro_use]
extern crate quote;

extern crate proc_macro;
use proc_macro::TokenStream;

use syn::{Body, Ident, Variant, VariantData};
use quote::Tokens;

#[proc_macro_derive(Node)]
pub fn my_macro(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let ast = syn::parse_macro_input(&input).unwrap();
    let name = &ast.ident;

    let quote_tokens = match ast.body {
        Body::Enum(ref data)   => gen_enum(name, data),
        Body::Struct(ref data) => gen_struct(name, data)
    }.to_string();

    format!("{} {}", input, quote_tokens.to_string()).parse().unwrap()
}

fn gen_enum(name: &Ident, data: &Vec<Variant>) -> Tokens {
    quote! { }
}

fn gen_struct(name: &Ident, data: &VariantData) -> Tokens {
    let match_property = gen_match_property(data);

    quote! {
        impl Node for #name {
            fn node_step(&mut self, mut runner: NodeRunner) -> String {
                match runner.step() {
                    NodeToken::ChainProperty (property) => {
                        #match_property
                    }
                    NodeToken::Get => {
                        format!("This is a {}", "struct")
                    }
                    action => { format!("Package cannot '{:?}'", action) }
                }
            }
        }
    }
}

fn gen_match_property(data: &VariantData) -> Tokens {
    let mut arms: Vec<Tokens> = vec!();
    for field in data.fields() {
        let name = &field.ident.as_ref().unwrap();
        let name_string = name.to_string();

        arms.push(quote!{
            #name_string => { self.#name.node_step(runner) }
        });
    }

    quote! {
        match property.as_str() {
            #( #arms ),*,
            prop  => format!("Package does not have a property '{}'", prop)
        }
    }
}
