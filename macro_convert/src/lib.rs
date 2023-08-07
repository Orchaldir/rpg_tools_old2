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
        let variants: Vec<Ident> = get_variants(data);
        let default: Ident = get_default(data);

        return quote! {
            use std::fmt;

            #[automatically_derived]
            impl #name {
                pub fn get_all() -> Vec<Self> {
                    vec![#(Self::#variants),*]
                }
            }

            #[automatically_derived]
            impl fmt::Display for #name {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "{:?}", self)
                }
            }

            #[automatically_derived]
            impl From<&str> for #name {
                fn from(string: &str) -> Self {
                    match string {
                        #(stringify!(#variants) => {Self::#variants})*
                        _ => {Self::#default}
                    }
                }
            }
        };
    }

    panic!("Expected a simple enum")
}

fn get_variants(data: &DataEnum) -> Vec<Ident> {
    data.variants.iter().map(|v| v.ident.clone()).collect()
}

fn get_default(data: &DataEnum) -> Ident {
    data.variants
        .iter()
        .find(|v| !v.attrs.is_empty())
        .unwrap_or_else(|| data.variants.iter().next().unwrap())
        .ident
        .clone()
}

fn is_simple_enum(data: &DataEnum) -> bool {
    for variant in &data.variants {
        if variant.fields != Fields::Unit {
            return false;
        }
    }

    true
}
