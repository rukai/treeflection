#![recursion_limit = "128"]
#[macro_use] extern crate quote;
             extern crate syn;
             extern crate proc_macro;
             extern crate proc_macro2;
             extern crate serde;
             extern crate serde_json;

use proc_macro::TokenStream;
use proc_macro2::Span;

use syn::{
    Attribute,
    Data,
    DeriveInput,
    Field,
    Fields,
    Ident,
    Meta,
    NestedMeta,
    Type,
    Variant,
    Visibility,
    Lit,
};
use syn::punctuated::{Iter, Punctuated};
use syn::token::Comma;
use quote::Tokens;

#[proc_macro_derive(Node, attributes(NodeActions))]
pub fn treeflection_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let actions = attrs_to_actions(&ast.attrs);

    let impl_for = match ast.data {
        Data::Enum(ref data)   => gen_enum(name, &data.variants, &actions),
        Data::Struct(ref data) => gen_struct(name, &data.fields, &actions),
        _ => unimplemented!()
    };
    let copy_var = gen_copy_var(name);

    let quote_tokens = quote! {
        #impl_for
        #copy_var
    };

    quote_tokens.into()
}

fn copy_var_name(name: &str) -> Ident {
    let mut var_name = name.to_uppercase();
    var_name.push_str("_COPY");
    Ident::from(var_name)
}

fn gen_copy_var(name: &Ident) -> Tokens {
    let var_name = copy_var_name(name.as_ref());
    quote! {
        static mut #var_name: Option<#name> = None;
    }
}

fn gen_get(name: &str) -> Tokens {
    quote_spanned!{ Span::call_site() =>
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
    quote_spanned!{ Span::call_site() =>
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

fn gen_copy(name: &str) -> Tokens {
    let var_name = copy_var_name(name);
    quote_spanned!{ Span::call_site() =>
        let copy = Some(self.clone());
        unsafe {
            #var_name = copy;
        }
        String::new()
    }
}

fn gen_paste(name: &str) -> Tokens {
    let var_name = copy_var_name(name);
    quote_spanned!{ Span::call_site() =>
        let paste = unsafe { #var_name.clone() };
        match paste {
            Some (value) => {
                *self = value;
                String::new()
            }
            None => {
                format!("{} has not been copied", #name)
            }
        }
    }
}

fn gen_custom_actions(name: &str, actions: &[Action]) -> Tokens {
    let mut arms: Vec<Tokens> = vec!();
    for action in actions {
        let action_name = &action.action;
        let function_name = Ident::from(action.function.as_ref());
        let mut args: Vec<Tokens> = vec!();
        for i in 0..action.args {
            args.push(quote_spanned! { Span::call_site() => args[#i].clone() });
        }

        let function_call = if action.return_string {
            quote_spanned!{ Span::call_site() =>
                self.#function_name(#( #args ),*)
            }
        } else {
            quote_spanned!{ Span::call_site() =>
                self.#function_name(#( #args ),*);
                String::new()
            }
        };

        arms.push(quote_spanned!{ Span::call_site() =>
            #action_name => {
                #function_call
            }
        });
    }

    quote_spanned!{ Span::call_site() =>
        match action.as_str() {
            #( #arms )*
            a => format!("{} cannot '{}'", #name, a)
        }
    }
}

fn gen_enum(name: &Ident, variants: &Punctuated<Variant, Comma>, actions: &[Action]) -> Tokens {
    let name_string = name.to_string();

    let property_arm = gen_enum_property(&name, variants.iter());
    let index_arm = gen_enum_index(&name, variants.iter());
    let get_arm = gen_get(&name_string);
    let set_arm = gen_set(&name_string);
    let copy_arm = gen_copy(&name_string);
    let paste_arm = gen_paste(&name_string);
    let help_arm = gen_enum_help(&name_string, variants.iter(), actions);
    let variant_arm = gen_variant(&name, variants.iter());
    let custom_arm = gen_custom_actions(&name_string, actions);
    let default_arm = quote_spanned!{ Span::call_site() =>
        *self = Default::default();
        String::new()
    };

    // this is required to avoid an unused variable warning from generated code
    let index_name = if check_using_index(variants.iter()) {
        quote_spanned!{ Span::call_site() => index }
    } else {
        quote_spanned!{ Span::call_site() => _ }
    };
    let args = if actions.iter().all(|x| x.args == 0) {
        quote_spanned!{ Span::call_site() => _ }
    } else {
        quote_spanned!{ Span::call_site() => args }
    };

    quote_spanned!{ Span::call_site() =>
        impl Node for #name {
            fn node_step(&mut self, mut runner: NodeRunner) -> String {
                match runner.step() {
                    NodeToken::ChainProperty (property) => { #property_arm }
                    NodeToken::ChainIndex (#index_name) => { #index_arm }
                    NodeToken::Get                      => { #get_arm }
                    NodeToken::Set (value)              => { #set_arm }
                    NodeToken::CopyFrom                 => { #copy_arm }
                    NodeToken::PasteTo                  => { #paste_arm }
                    NodeToken::Help                     => { #help_arm }
                    NodeToken::SetVariant (variant)     => { #variant_arm }
                    NodeToken::SetDefault               => { #default_arm }
                    NodeToken::Custom (action, #args)   => { #custom_arm }
                    action                              => { format!("{} cannot '{:?}'", #name_string, action) }
                }
            }
        }
    }
}

fn check_using_index(variants: Iter<Variant, Comma>) -> bool {
    for variant in variants {
        if let Fields::Unnamed (_) = variant.fields {
            return true;
        }
    }
    false
}

fn gen_variant(name: &Ident, variants: Iter<Variant, Comma>) -> Tokens {
    let name_string = name.to_string();
    let mut variant_arms: Vec<Tokens> = vec!();
    for variant in variants {
        let variant_name = &variant.ident;
        let variant_name_string = variant_name.to_string();
        variant_arms.push(match &variant.fields {
            &Fields::Named (ref fields) => {
                let mut field_values: Vec<Tokens> = vec!();
                for field in fields.named.iter() {
                    let field_name = field.ident.as_ref().unwrap();
                    field_values.push(quote_spanned!{ Span::call_site() =>
                        #field_name : Default::default()
                    });
                }
                quote_spanned!{ Span::call_site() =>
                    #variant_name_string => {
                        *self = #name::#variant_name { #( #field_values ),* };
                        String::new()
                    }
                }
            }
            &Fields::Unnamed (ref fields) => {
                let field_values: Vec<_> = fields.unnamed.iter().map(|_|
                    quote_spanned!{ Span::call_site() =>
                        Default::default()
                    }
                ).collect();
                quote_spanned!{ Span::call_site() =>
                    #variant_name_string => {
                        *self = #name::#variant_name ( #( #field_values ),* );
                        String::new()
                    }
                }
            }
            &Fields::Unit => {
                quote_spanned!{ Span::call_site() =>
                    #variant_name_string => {
                        *self = #name::#variant_name;
                        String::new()
                    }
                }
            }
        });
    }

    quote_spanned!{ Span::call_site() =>
        match variant.as_str() {
            #( #variant_arms )*
            variant => format!("{} does not have a variant '{}'", #name_string, variant)
        }
    }
}

fn gen_enum_property(name: &Ident, variants: Iter<Variant, Comma>) -> Tokens {
    let mut enum_arms: Vec<Tokens> = vec!();

    for variant in variants {
        let variant_name = &variant.ident;
        let variant_name_string = &variant.ident.to_string();
        enum_arms.push(match &variant.fields {
            &Fields::Named(ref fields) => {
                let mut field_names: Vec<Tokens> = vec!();
                let mut property_arms: Vec<Tokens> = vec!();
                for field in fields.named.iter() {
                    let field_name = &field.ident;
                    let field_name_string = field_name.as_ref().unwrap().to_string();
                    field_names.push(quote_spanned!{ Span::call_site() => ref mut #field_name });

                    property_arms.push(quote_spanned!{ Span::call_site() => #field_name_string => { #field_name.node_step(runner) } });
                }

                quote_spanned!{ Span::call_site() =>
                    &mut #name::#variant_name { #( #field_names ),* } => {
                        match property.as_str() {
                            #( #property_arms )*
                            _ => { format!("{} does not have a property '{}'", #variant_name_string, property) }
                        }
                    }
                }
            }
            &Fields::Unnamed (ref fields) => {
                let mut underscores: Vec<Tokens> = vec!();
                for _ in fields.unnamed.iter() {
                    underscores.push(quote_spanned!{ Span::call_site() => _});
                }

                quote_spanned!{ Span::call_site() =>
                    &mut #name::#variant_name ( #( #underscores ),* ) => { format!("{} does not have a property '{}'", #variant_name_string, property) }
                }
            }
            &Fields::Unit => {
                quote_spanned!{ Span::call_site() =>
                    &mut #name::#variant_name => { format!("{} does not have a property '{}'", #variant_name_string, property) }
                }
            }
        });
    }

    quote_spanned!{ Span::call_site() =>
        match self {
            #( #enum_arms )*
        }
    }
}

fn gen_enum_index(name: &Ident, variants: Iter<Variant, Comma>) -> Tokens {
    let mut enum_arms: Vec<Tokens> = vec!();

    for variant in variants {
        let variant_name = &variant.ident;
        let variant_name_string = &variant.ident.to_string();
        enum_arms.push(match &variant.fields {
            &Fields::Named (ref fields) => {
                let mut name_pairs: Vec<Tokens> = vec!();
                for field in fields.named.iter() {
                    let field_name = &field.ident;
                    name_pairs.push(quote_spanned!{ Span::call_site() => #field_name: _ });
                }

                quote_spanned!{ Span::call_site() =>
                    &mut #name::#variant_name { #( #name_pairs ),* } => { format!("Cannot index {}", #variant_name_string) }
                }
            }
            &Fields::Unnamed (ref fields) => {
                let mut tuple_names: Vec<Tokens> = vec!();
                let mut index_arms: Vec<Tokens> = vec!();
                for i in 0..fields.unnamed.len() {
                    let tuple_name = Ident::from(format!("x{}", i));
                    tuple_names.push(quote_spanned!{ Span::call_site() => ref mut #tuple_name });
                    index_arms.push(quote!{ #i => { #tuple_name.node_step(runner) } });
                }
                let highest_index = fields.unnamed.len()-1;

                quote_spanned!{ Span::call_site() =>
                    &mut #name::#variant_name ( #( #tuple_names ),* ) => {
                        match index {
                            #( #index_arms ),*
                            _ => { format!("Used index {} on a {} (try a value between 0-{}", index, #variant_name_string, #highest_index ) }
                        }
                    }
                }
            }
            &Fields::Unit => {
                quote_spanned!{ Span::call_site() =>
                    &mut #name::#variant_name => { format!("Cannot index {}", #variant_name_string) }
                }
            }
        });
    }

    quote_spanned!{ Span::call_site() =>
        match self {
            #( #enum_arms )*
        }
    }
}

fn gen_struct(name: &Ident, data: &Fields, actions: &[Action]) -> Tokens {
    let name_string = name.to_string();
    match data {
        &Fields::Named(ref fields_named) => {
            let property_arm = gen_struct_property(&name_string, fields_named.named.iter());
            let get_arm = gen_get(&name_string);
            let set_arm = gen_set(&name_string);
            let copy_arm = gen_copy(&name_string);
            let paste_arm = gen_paste(&name_string);
            let help_arm = gen_struct_help(&name_string, fields_named.named.iter(), actions);
            let custom_arm = gen_custom_actions(&name_string, actions);
            let default_arm = quote_spanned!{ Span::call_site() =>
                *self = Default::default();
                String::new()
            };

            // this is required to avoid an unused variable warning from generated code
            let args = if actions.iter().all(|x| x.args == 0) {
                quote_spanned!{ Span::call_site() => _ }
            } else {
                quote_spanned!{ Span::call_site() => args }
            };

            quote_spanned! { Span::call_site() =>
                impl Node for #name {
                    fn node_step(&mut self, mut runner: NodeRunner) -> String {
                        match runner.step() {
                            NodeToken::ChainProperty (property) => { #property_arm }
                            NodeToken::Get                      => { #get_arm }
                            NodeToken::Set (value)              => { #set_arm }
                            NodeToken::CopyFrom                 => { #copy_arm }
                            NodeToken::PasteTo                  => { #paste_arm }
                            NodeToken::Help                     => { #help_arm }
                            NodeToken::SetDefault               => { #default_arm }
                            NodeToken::Custom (action, #args)   => { #custom_arm }
                            action                              => { format!("{} cannot '{:?}'", #name_string, action) }
                        }
                    }
                }
            }
        }
        _ => unimplemented!()
    }
}

fn gen_struct_property(name: &str, fields: Iter<Field, Comma>) -> Tokens {
    let mut arms: Vec<Tokens> = vec!();
    for field in fields {
        let field_name = &field.ident.as_ref().unwrap();
        let field_name_string = field_name.to_string();
        if let Visibility::Public(_) = field.vis {
            arms.push(quote_spanned!{ Span::call_site() =>
                #field_name_string => { self.#field_name.node_step(runner) },
            });
        }
    }

    quote_spanned!{ Span::call_site() =>
        match property.as_str() {
            #( #arms )*
            prop => format!("{} does not have a property '{}'", #name, prop)
        }
    }
}

fn gen_struct_help(name: &str, fields: Iter<Field, Comma>, actions: &[Action]) -> Tokens {
    let mut output = format!(r#"
{} Help

Actions:
*   help  - display this help
*   get   - display JSON
*   set   - set to JSON
*   copy  - copy the values from this struct
*   paste - paste the copied values to this struct
*   reset - reset to default values
{}
Accessors:
"#, name, custom_action_help(actions));

    for field in fields {
        if let Visibility::Public(_) = field.vis {
            let field_name = &field.ident.as_ref().unwrap();
            let field_type = type_string(&field.ty);
            output.push_str(format!("*   {} - {}\n", field_name, field_type).as_ref());
        }
    }
    output.pop();

    quote_spanned!{ Span::call_site() =>
        String::from(#output)
    }
}

fn gen_enum_help(name: &str, variants: Iter<Variant, Comma>, actions: &[Action]) -> Tokens {
    let mut variant_list = String::new();
    let mut accessor_list = String::new();

    for variant in variants {
        let variant_name = &variant.ident.as_ref();
        variant_list.push_str(format!("*   {}\n", variant_name).as_ref());
        match &variant.fields {
            &Fields::Named (ref fields) => {
                accessor_list.push_str(format!("As {}:\n", variant_name).as_ref());
                for field in fields.named.iter() {
                    let field_name = field.ident.as_ref().unwrap().as_ref();
                    let field_type = type_string(&field.ty);
                    accessor_list.push_str(format!("*   .{} - {}\n", field_name, field_type).as_ref());
                }
            }
            &Fields::Unnamed (ref fields) => {
                accessor_list.push_str(format!("As {}:\n", variant_name).as_ref());
                for (i, field) in fields.unnamed.iter().enumerate() {
                    let field_type = type_string(&field.ty);
                    accessor_list.push_str(format!("*   [{}] - {}\n", i, field_type).as_ref());
                }
            }
            &Fields::Unit => { }
        }
    }

    let accessor_info = if accessor_list.is_empty() {
        String::new()
    } else {
        String::from("Accessors:\nChanges depending on which variant the enum is currently set to:\n")
    };

    let custom_actions = custom_action_help(actions);

    let output = format!(r#"
{} Help

Actions:
*   help    - display this help
*   get     - display JSON
*   set     - set to JSON
*   copy    - copy the values from this enum
*   paste   - paste the copied values to this enum
*   reset   - reset to default variant
*   variant - set to the specified variant
{}
Valid variants:
{}
{}
{}"#, name, custom_actions, variant_list, accessor_info, accessor_list);

    quote_spanned!{ Span::call_site() =>
        String::from(#output)
    }
}

fn custom_action_help(actions: &[Action]) -> String {
    let mut result = String::new();
    for action in actions {
        let action_string = if let &Some(ref help) = &action.help {
            format!("*   {} - {}\n", action.action, help)
        } else {
            format!("*   {}\n", action.action)
        };
        result.push_str(action_string.as_ref());
    }
    result
}

fn type_string(ty: &Type) -> String {
    match ty {
        &Type::Path (ref path) => {
            String::from(path.path.segments[0].ident.as_ref())
        }
        &Type::Tuple (_) => String::from("Tuple"),
        _ => String::from("UNABLE TO GET TYPE")
    }
}

fn attrs_to_actions(attrs: &[Attribute]) -> Vec<Action> {
    let mut actions: Vec<Action> = vec!();
    for attr in attrs {
        if let Some(Meta::List (list)) = attr.interpret_meta() {
            if list.ident == "NodeActions" {
                for nest_meta in list.nested.iter() {
                    if let &NestedMeta::Meta (ref sub_attr) = nest_meta {
                        actions.push(attr_to_action(sub_attr));
                    }
                    else {
                        panic!("Invalid NodeActions attribute: Needs to be a list of NodeAction")
                    }
                }
            }
        }
    }
    actions
}

fn attr_to_action(attr: &Meta) -> Action {
    if let &Meta::List (ref list) = attr {
        if list.ident == "NodeAction" {
            let mut action: Option<String> = None;
            let mut function: Option<String> = None;
            let mut args: usize = 0;
            let mut return_string = false;
            let mut help: Option<String> = None;
            for nest_meta in list.nested.iter() {
                if let &NestedMeta::Meta (ref meta) = nest_meta {
                    match meta {
                        &Meta::Word (ref ident) => {
                            if ident == "return_string" {
                                return_string = true;
                            } else {
                                panic!("Invalid NodeAction attribute: Invalid value in list");
                            }
                        }
                        &Meta::NameValue (ref name_value) => {
                            match name_value.ident.as_ref() {
                                "action" => {
                                    if let &Lit::Str(ref lit) = &name_value.lit { action = Some(lit.value()) }
                                    else { panic!("Invalid NodeAction attribute: Expected a string for action value"); }
                                }
                                "function" => {
                                    if let &Lit::Str(ref lit) = &name_value.lit { function = Some(lit.value()) }
                                    else { panic!("Invalid NodeAction attribute: Expected a string for function value"); }
                                }
                                "help" => {
                                    if let &Lit::Str(ref lit) = &name_value.lit { help = Some(lit.value()) }
                                    else { panic!("Invalid NodeAction attribute: Expected a string for help value"); }
                                }
                                "args" => {
                                    if let &Lit::Str(ref lit) = &name_value.lit {
                                        args = lit.value().parse::<usize>().expect("Invalid NodeAction attribute: Expected a string that can parse into usize");
                                    }
                                    else {
                                        panic!("Invalid NodeAction attribute: Expected a string for args value");
                                    }
                                }
                                _ => { panic!("Invalid NodeAction attribute: Invalid value in list"); }
                            }
                        }
                        &Meta::List (_) => {
                            panic!("Invalid NodeAction attribute: Invalid value in list");
                        }
                    }
                }
                else {
                    panic!("Invalid NodeAction attribute: Invalid value in list");
                }
            }

            let function = function.expect("Invalid NodeAction attribute: Needs to specify a function");

            Action {
                action:        action.unwrap_or(function.clone()),
                function:      function,
                args:          args,
                return_string: return_string,
                help:          help,
            }
        }
        else {
            panic!("Invalid NodeAction attribute: Needs to be a list")
        }
    }
    else {
        panic!("Invalid NodeAction attribute: Needs to be a list")
    }
}

struct Action {
    pub action:       String,
    pub function:     String,
    pub args:         usize,
    pub return_string: bool,
    pub help:         Option<String>,
}
