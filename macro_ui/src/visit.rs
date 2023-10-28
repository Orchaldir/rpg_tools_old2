use crate::utils::{get_field_type, get_option_type, is_integer, is_option};
use quote::quote;
use syn::__private::TokenStream2;
use syn::{DataEnum, Field, Fields};

pub fn visit_enum_variants(data: &DataEnum) -> TokenStream2 {
    let mut results: Vec<TokenStream2> = Vec::new();

    for variant in &data.variants {
        let variant_name = &variant.ident;
        results.push(quote! {  visitor.enter_tuple_variant(stringify!(#variant_name)); });

        match &variant.fields {
            Fields::Named(fields) => {
                results.push(quote! {  println!("{}Add named variant '{}'!", &inner_spaces, stringify!(#variant_name)); });

                for field in &fields.named {
                    results.push(visit_struct_field(field));
                }
            }
            Fields::Unnamed(fields) => {
                if fields.unnamed.len() != 1 {
                    panic!("Tuple enums are only supported with 1 field!")
                }

                results.push(quote! {  println!("{}Add unnamed variant '{}'!", &inner_spaces, stringify!(#variant_name)); });

                results.push(visit_tuple_field(&fields.unnamed[0], "c"));
            }
            Fields::Unit => {
                results.push(quote! {
                    println!("{}Add simple variant '{}'!", &inner_spaces, stringify!(#variant_name));
                    visitor.add_unit_variant(stringify!(#variant_name));
                });
            }
        }
    }

    quote! { #(#results)* }
}

pub fn visit_struct_field(field: &Field) -> TokenStream2 {
    let field_name = &field.ident;

    if is_integer(field) {
        quote! {
            println!("{}Add integer '{}'!", &inner_spaces, stringify!(#field_name));
            visitor.add_integer(stringify!(#field_name));
        }
    } else if is_option(field) {
        let option_type = &get_option_type(field);
        quote! {
            println!("{}Add option for '{}'!", &inner_spaces, stringify!(#option_type));
            visitor.enter_child(stringify!(#field_name));
            visit_option::<#option_type>(visitor, &inner_spaces);
            visitor.leave_child();
        }
    } else {
        let name = &get_field_type(field);
        quote! {
            visitor.enter_child(stringify!(#field_name));
            #name::visit(visitor, &inner_spaces, false);
            visitor.leave_child();
        }
    }
}

fn visit_tuple_field(field: &Field, field_name: &str) -> TokenStream2 {
    if is_integer(field) {
        quote! {
            println!("{}Add integer '{}'!", &inner_spaces, #field_name);
            visitor.add_integer(#field_name);
        }
    } else {
        let name = &get_field_type(field);
        quote! {
            visitor.enter_child(#field_name);
            #name::visit(visitor, &inner_spaces, true);
            visitor.leave_child();
        }
    }
}
