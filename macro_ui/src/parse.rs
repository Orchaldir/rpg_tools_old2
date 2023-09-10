use crate::utils::{get_field_type, is_integer};
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::{DataEnum, Field, Fields, Ident};

pub fn parse_enum_variants(name: &Ident, data: &DataEnum) -> TokenStream2 {
    let mut results: Vec<TokenStream2> = Vec::new();

    for variant in &data.variants {
        let variant_name = &variant.ident;

        let variant_result = match &variant.fields {
            Fields::Named(fields) => {
                let parsed_fields: TokenStream2 =
                    fields.named.iter().map(parse_struct_field).collect();

                quote! {
                    #name::#variant_name {
                        #parsed_fields
                    }
                }
            }
            Fields::Unnamed(fields) => {
                if fields.unnamed.len() != 1 {
                    panic!("Tuple enums are only supported with 1 field!")
                }

                let tuple_result = parse_tuple_field(&fields.unnamed[0], "c");

                quote! {  #name::#variant_name(#tuple_result) }
            }
            Fields::Unit => {
                quote! {  #name::#variant_name }
            }
        };

        results.push(quote! {  stringify!(#variant_name) => #variant_result, });
    }

    quote! { #(#results)* }
}

pub fn parse_struct_field(field: &Field) -> TokenStream2 {
    let field_name = &field.ident;

    if is_integer(field) {
        quote! {
            #field_name: parser.parse_u32(&format!("{}.{}", path, stringify!(#field_name)), 0),
        }
    } else {
        let name = &get_field_type(field);
        quote! {
            #field_name: #name::parse(parser, &format!("{}.{}", path, stringify!(#field_name)), &format!("  {}", spaces)),
        }
    }
}

pub fn parse_tuple_field(field: &Field, field_name: &str) -> TokenStream2 {
    if is_integer(field) {
        quote! {
            parser.parse_u32(&format!("{}.{}", path, #field_name), 0)
        }
    } else {
        let name = &get_field_type(field);
        quote! {
            #name::parse(parser, &format!("{}.{}", path, #field_name), &format!("  {}", spaces))
        }
    }
}
