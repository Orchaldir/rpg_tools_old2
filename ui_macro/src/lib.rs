use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, Fields};

#[proc_macro_derive(ui)]
pub fn ui_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_ui_macro(&ast)
}

fn impl_ui_macro(input: &syn::DeriveInput) -> TokenStream {
    println!("input={:?}", input);

    let name = &input.ident;

    println!("name={:?}", name);

    let gen = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => {
            let field_name = fields.named.iter().map(|field| &field.ident);
            let field_type = fields.named.iter().map(|field| &field.ty);

            quote! {
                impl UI for #name {
                    fn create_viewer(&self, path: &str) {
                        println!("Create Viewer for {} with path '{}'!", stringify!(#name), path);
                        #(
                            println!("Add field {} of type {}!", stringify!(#field_name), stringify!(#field_type));
                        )*
                    }
                }
            }
        }
        _ => panic!("Expected a struct with named fields"),
    };

    gen.into()
}
