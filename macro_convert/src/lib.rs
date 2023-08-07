use proc_macro::TokenStream;
use quote::quote;
use syn::__private::TokenStream2;
use syn::{Data, DataEnum, Fields, Ident};

#[proc_macro_derive(Convert)]
pub fn convert_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_convert_macro(&ast)
}

fn impl_convert_macro(input: &syn::DeriveInput) -> TokenStream {
    let name = &input.ident;

    let gen = match &input.data {
        Data::Enum(data) => handle_enum(name, data),
        _ => panic!("Expected an enum"),
    };

    gen.into()
}

fn handle_enum(name: &Ident, data: &DataEnum) -> TokenStream2 {
    if is_simple_enum(data) {
        return quote! {
            /// Test Convert Macro
        };
    }

    panic!("Expected a simple enum")
}

fn is_simple_enum(data: &DataEnum) -> bool {
    for variant in &data.variants {
        if variant.fields != Fields::Unit {
            return false;
        }
    }

    true
}
