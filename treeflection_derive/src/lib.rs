#![feature(proc_macro, proc_macro_lib)]

#[macro_use] extern crate quote;

extern crate syn;
extern crate regex;
extern crate proc_macro;
extern crate serde;
extern crate serde_json;

use proc_macro::TokenStream;

use syn::{Body, Ident, Variant, VariantData, Visibility, Field, Ty};
use quote::Tokens;

#[proc_macro_derive(Node)]
pub fn treeflection_derive(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let ast = syn::parse_macro_input(&input).unwrap();
    let name = &ast.ident;

    let quote_tokens = match ast.body {
        Body::Enum(ref data)   => gen_enum(name, data),
        Body::Struct(ref data) => gen_struct(name, data)
    }.to_string();

    quote_tokens.to_string().parse().unwrap()
}

fn gen_get(name: &str) -> Tokens {
    quote! {
        use serde_json;
        match serde_json::to_string_pretty(self) {
            Ok(result) => {
                result
            }
            Err(err) => {
                format!("{} get Error: {}", #name, err)
            }
        }
    }
}

fn gen_set(name: &str) -> Tokens {
    quote! {
        use serde_json;
        match serde_json::from_str(value.as_str()) {
            Ok(result) => {
                *self = result;
                String::from("")
            }
            Err(err) => {
                format!("{} set Error: {}", #name, err)
            }
        }
    }
}

fn gen_enum(name: &Ident, data: &Vec<Variant>) -> Tokens {
    let name_string = name.to_string();

    let property_arm = gen_enum_property(&name, data);
    let index_arm = gen_enum_index(&name, data);
    let get_arm = gen_get(&name_string);
    let set_arm = gen_set(&name_string);
    let help_arm = gen_enum_help(&name_string, data);

    quote! {
        impl Node for #name {
            fn node_step(&mut self, mut runner: NodeRunner) -> String {
                match runner.step() {
                    NodeToken::ChainProperty (property) => { #property_arm }
                    NodeToken::ChainIndex (index)       => { #index_arm }
                    NodeToken::Get                      => { #get_arm }
                    NodeToken::Set (value)              => { #set_arm }
                    NodeToken::Help                     => { #help_arm }
                    action                              => { format!("{} cannot '{:?}'", #name_string, action) }
                }
            }
        }
    }
}

fn gen_enum_property(name: &Ident, data: &Vec<Variant>) -> Tokens {
    let mut enum_arms: Vec<Tokens> = vec!();

    for variant in data {
        let variant_name = &variant.ident;
        let variant_name_string = &variant.ident.to_string();
        enum_arms.push(match &variant.data {
            &VariantData::Struct (ref fields) => {
                let mut field_names: Vec<Tokens> = vec!();
                let mut property_arms: Vec<Tokens> = vec!();
                for field in fields {
                    let field_name = &field.ident;
                    let field_name_string = field_name.as_ref().unwrap().to_string();
                    field_names.push(quote!{ ref mut #field_name });

                    property_arms.push(quote!{ #field_name_string => { #field_name.node_step(runner) } });
                }

                quote! {
                    &mut #name::#variant_name { #( #field_names ),* } => {
                        match property.as_str() {
                            #( #property_arms )*
                            _ => { format!("{} does not have a property '{}'", #variant_name_string, property) }
                        }
                    }
                }
            }
            &VariantData::Tuple (ref fields) => {
                let mut underscores: Vec<Tokens> = vec!();
                for _ in fields {
                    underscores.push(quote!{_});
                }

                quote! {
                    &mut #name::#variant_name ( #( #underscores ),* ) => { format!("{} does not have a property '{}'", #variant_name_string, property) }
                }
            }
            &VariantData::Unit => {
                quote! {
                    &mut #name::#variant_name => { format!("{} does not have a property '{}'", #variant_name_string, property) }
                }
            }
        });
    }

    quote! {
        match self {
            #( #enum_arms )*
        }
    }
}

fn gen_enum_index(name: &Ident, data: &Vec<Variant>) -> Tokens {
    let mut enum_arms: Vec<Tokens> = vec!();

    for variant in data {
        let variant_name = &variant.ident;
        let variant_name_string = &variant.ident.to_string();
        enum_arms.push(match &variant.data {
            &VariantData::Struct (ref fields) => {
                let mut name_pairs: Vec<Tokens> = vec!();
                for field in fields {
                    let field_name = &field.ident;
                    name_pairs.push(quote!{ #field_name: _ });
                }

                quote! {
                    &mut #name::#variant_name { #( #name_pairs ),* } => { format!("Cannot index {}", #variant_name_string) }
                }
            }
            &VariantData::Tuple (ref fields) => {
                let mut tuple_names: Vec<Tokens> = vec!();
                let mut index_arms: Vec<Tokens> = vec!();
                for i in 0..fields.len() {
                    let tuple_name = Ident::from(format!("x{}", i));
                    tuple_names.push(quote!{ ref mut #tuple_name });
                    index_arms.push(quote!{ #i => { #tuple_name.node_step(runner) } });
                }
                let highest_index = fields.len()-1;

                quote! {
                    &mut #name::#variant_name ( #( #tuple_names ),* ) => {
                        match index {
                            #( #index_arms ),*
                            _ => { format!("Used index {} on a {} (try a value between 0-{}", index, #variant_name_string, #highest_index ) }
                        }
                    }
                }
            }
            &VariantData::Unit => {
                quote! {
                    &mut #name::#variant_name => { format!("Cannot index {}", #variant_name_string) }
                }
            }
        });
    }

    quote! {
        match self {
            #( #enum_arms )*
        }
    }
}

fn gen_struct(name: &Ident, data: &VariantData) -> Tokens {
    let name_string = name.to_string();

    let property_arm = gen_struct_property(&name_string, data);
    let get_arm = gen_get(&name_string);
    let set_arm = gen_set(&name_string);
    let help_arm = gen_struct_help(&name_string, data);

    quote! {
        impl Node for #name {
            fn node_step(&mut self, mut runner: NodeRunner) -> String {
                match runner.step() {
                    NodeToken::ChainProperty (property) => { #property_arm }
                    NodeToken::Get                      => { #get_arm }
                    NodeToken::Set (value)              => { #set_arm }
                    NodeToken::Help                     => { #help_arm }
                    action                              => { format!("{} cannot '{:?}'", #name_string, action) }
                }
            }
        }
    }
}

fn gen_struct_property(name: &str, data: &VariantData) -> Tokens {
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

fn gen_struct_help(name: &str, data: &VariantData) -> Tokens {
    let mut output = format!(r#"
{} Help

Commands:
*   help - display this help
*   get  - display JSON
*   set  - set to JSON

Accessors:
"#, name);

    for field in data.fields() {
        if let Visibility::Public = field.vis {
            let field_name = &field.ident.as_ref().unwrap();
            let field_type = get_field_type(field);
            output.push_str(format!("*   {} - {}\n", field_name, field_type).as_ref());
        }
    }
    output.pop();

    quote!{
        String::from(#output)
    }
}

fn gen_enum_help(name: &str, data: &Vec<Variant>) -> Tokens {
    let mut valid_values = String::new();
    for field in data {
        let field_name = &field.ident.as_ref();
        valid_values.push_str(format!("*   {}\n", field_name).as_ref());
    }

    let output = format!(r#"
{} Help

Valid values:
{}
Commands:
*   help - display this help
*   get  - display JSON
*   set  - set to JSON"#, name, valid_values);

    quote!{
        String::from(#output)
    }
}

fn get_field_type(field: &Field) -> &str {
    if let Ty::Path (_, ref path) = field.ty {
        // TODO: Do I want the first or last segment?
        for segment in &path.segments {
            return segment.ident.as_ref();
        }
    }
    "UNABLE TO GET TYPE"
}
