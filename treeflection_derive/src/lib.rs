#![feature(proc_macro, proc_macro_lib)]

extern crate syn;
#[macro_use]
extern crate quote;

extern crate proc_macro;
use proc_macro::TokenStream;

use syn::{Body, Ident, Variant, VariantData, Visibility};
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
    let name_string = name.to_string();
    let match_property = gen_match_property(&name_string, data);

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
                    action => { format!("{} cannot '{:?}'", #name_string, action) }
                }
            }
        }
    }
}

fn gen_match_property(name: &str, data: &VariantData) -> Tokens {
    let mut arms: Vec<Tokens> = vec!();
    for field in data.fields() {
        let field_name = &field.ident.as_ref().unwrap();
        let field_name_string = field_name.to_string();
        if let Visibility::Public = field.vis {
            arms.push(quote!{
                #field_name_string => { self.#field_name.node_step(runner) }
            });
        }
    }

    quote! {
        match property.as_str() {
            #( #arms ),*,
            prop  => format!("{} does not have a property '{}'", #name, prop)
        }
    }
}
