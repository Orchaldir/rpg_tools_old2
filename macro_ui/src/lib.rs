use crate::parse::{parse_enum_variants, parse_struct_field};
use crate::utils::is_simple_enum;
use crate::visit::{visit_enum_variants, visit_struct_field};
use proc_macro::TokenStream;
use quote::quote;
use syn::__private::TokenStream2;
use syn::{Data, DataEnum, DataStruct, Fields, FieldsNamed, Ident};

extern crate macro_core;

mod parse;
mod utils;
mod visit;

#[proc_macro_derive(ui)]
pub fn ui_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_ui_macro(&ast)
}

fn impl_ui_macro(input: &syn::DeriveInput) -> TokenStream {
    let name = &input.ident;

    let gen = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => handle_struct(name, fields),
        Data::Enum(data) => handle_enum(name, data),
        _ => panic!("Expected a struct with named fields"),
    };

    gen.into()
}

fn handle_enum(name: &Ident, data: &DataEnum) -> TokenStream2 {
    let variants: Vec<Ident> = data.variants.iter().map(|v| v.ident.clone()).collect();

    if is_simple_enum(data) {
        return quote! {
            #[automatically_derived]
            impl macro_core::visitor::UI for #name {
                fn visit(visitor: &mut dyn macro_core::visitor::UiVisitor, spaces: &str, _in_tuple: bool) {
                    println!("{}Add simple enum {} with path '{}'!", spaces, stringify!(#name), visitor.get_path());
                    visitor.add_simple_enum(&[#(stringify!(#variants).to_string()),*]);
                }
            }

            #[automatically_derived]
            impl #name {
                pub fn parse<'a>(parser: &'a dyn macro_core::parser::UiParser<'a>, path: &str, spaces: &str) -> #name {
                    println!("{}Parse simple enum {} with path '{}'", spaces, stringify!(#name), path);
                    macro_core::parser::get_enum(parser, path)
                }
            }
        };
    }

    let visited_fields = visit_enum_variants(data);
    let parsed_fields = parse_enum_variants(name, data);

    quote! {
        #[automatically_derived]
        impl macro_core::visitor::UI for #name {
            fn visit(visitor: &mut dyn macro_core::visitor::UiVisitor, spaces: &str, _in_tuple: bool) {
                println!("{}Create Viewer for enum {} with path '{}'!", spaces, stringify!(#name), visitor.get_path());
                visitor.enter_enum(&[#(stringify!(#variants).to_string()),*]);
                let inner_spaces = format!("  {}", spaces);
                #visited_fields
                visitor.leave_enum();
                println!("{}Finish Viewer for enum {} with path '{}'!", spaces, stringify!(#name), visitor.get_path());
            }
        }

        #[automatically_derived]
        impl #name {
            pub fn parse<'a>(parser: &'a dyn macro_core::parser::UiParser<'a>, path: &str, spaces: &str) -> #name {
                println!("{}Parse complex enum {} with path '{}'", spaces, stringify!(#name), path);
                let t = parser.get_str(&format!("{}.type", path)).unwrap_or("");
                println!("{}type '{}'", spaces, t);

                match t {

                    #parsed_fields

                    _ => #name::default(),
                }
            }
        }
    }
}

fn handle_struct(name: &Ident, fields: &FieldsNamed) -> TokenStream2 {
    let visited_fields: TokenStream2 = fields.named.iter().map(visit_struct_field).collect();
    let parsed_fields: TokenStream2 = fields.named.iter().map(parse_struct_field).collect();

    quote! {
        #[automatically_derived]
        impl macro_core::visitor::UI for #name {
            fn visit(visitor: &mut dyn macro_core::visitor::UiVisitor, spaces: &str, in_tuple: bool) {
                println!("{}Create Viewer for struct {} with path '{}' & in_tuple={}!", spaces, stringify!(#name), visitor.get_path(), in_tuple);
                visitor.enter_struct(in_tuple);
                let inner_spaces = format!("  {}", spaces);
                #visited_fields
                visitor.leave_struct(in_tuple);
                println!("{}Finish Viewer for struct {} with path '{}'!", spaces, stringify!(#name), visitor.get_path());
            }
        }

        #[automatically_derived]
        impl #name {
            pub fn parse<'a>(parser: &'a dyn macro_core::parser::UiParser<'a>, path: &str, spaces: &str) -> #name {
                println!("{}Parse struct {} with path '{}'", spaces, stringify!(#name), path);
                Self {
                    #parsed_fields
                }
            }
        }
    }
}
