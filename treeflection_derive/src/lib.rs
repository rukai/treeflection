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

    //println!("{}", quote_tokens.to_string());
    format!("{} {}", input, quote_tokens.to_string()).parse().unwrap()
}

fn gen_enum(name: &Ident, data: &Vec<Variant>) -> Tokens {
    let match_get = gen_enum_get(name, data);
    let match_set = gen_enum_set(name, data);
    let name_string = name.to_string();
    quote! {
        impl Node for #name {
            fn node_step(&mut self, mut runner: NodeRunner) -> String {
                match runner.step() {
                    NodeToken::Get => {
                        #match_get
                    }
                    NodeToken::Set (value) => {
                        #match_set
                    }
                    action => { format!("{} cannot '{:?}'", #name_string, action) }
                }
            }
        }
    }
}

fn gen_enum_get(enum_name: &Ident, data: &Vec<Variant>) -> Tokens {
    let mut match_arms: Vec<Tokens> = vec!();
    for variant in data {
        let name = &variant.ident;
        let name_string = name.to_string();
        match_arms.push(match variant.data {
            VariantData::Unit => {
                quote! { &mut #enum_name::#name => String::from(#name_string), }
            }
            VariantData::Tuple(ref fields) => {
                let mut tuple_args: Vec<Tokens> = vec!();
                let mut format_args: Vec<Tokens> = vec!();
                let mut format_string = name.to_string();
                format_string.push_str("(");

                for (i, field) in fields.iter().enumerate() {
                    let arg = Ident::from(format!("v{}", i));

                    tuple_args.push(quote!( ref #arg ));
                    format_args.push(quote!( #arg ));

                    format_string.push_str("{}, ");
                }
                format_string.pop();
                format_string.pop();
                format_string.push_str(")");

                quote! { &mut #enum_name::#name( #( #tuple_args ),*) => format!(#format_string, #( #format_args ),*), }
            }
            VariantData::Struct(ref fields) => {
                quote! { }
            }
        });
    }
    quote! {
        match self {
            #( #match_arms )*
        }
    }
}

fn gen_enum_set(enum_name: &Ident, data: &Vec<Variant>) -> Tokens {
    let enum_name_string = enum_name.to_string();
    let mut match_arms: Vec<Tokens> = vec!();
    for variant in data {
        let name = &variant.ident;
        let name_string = name.to_string();
        match_arms.push(match variant.data {
            VariantData::Unit => {
                quote! { #name_string => { *self = #enum_name::#name; String::from("") },}
            }
            VariantData::Tuple(ref fields) => {
                quote! { }
            }
            VariantData::Struct(ref fields) => {
                quote! { }
            }
        });
    }
    quote! {
        match value.as_ref() {
            #( #match_arms )*
            value_miss => { format!("{} is not a valid value for {}", value_miss, #enum_name_string) }
        }
    }
}

fn gen_struct(name: &Ident, data: &VariantData) -> Tokens {
    let name_string = name.to_string();
    let match_property = gen_struct_chain_property(&name_string, data);

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

fn gen_struct_chain_property(name: &str, data: &VariantData) -> Tokens {
    let mut arms: Vec<Tokens> = vec!();
    for field in data.fields() {
        let field_name = &field.ident.as_ref().unwrap();
        let field_name_string = field_name.to_string();
        if let Visibility::Public = field.vis {
            arms.push(quote!{
                #field_name_string => { self.#field_name.node_step(runner) },
            });
        }
    }

    quote! {
        match property.as_str() {
            #( #arms )*
            prop  => format!("{} does not have a property '{}'", #name, prop)
        }
    }
}
