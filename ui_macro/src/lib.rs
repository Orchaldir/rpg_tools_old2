use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::{Data, DataStruct, Field, Fields, Type};

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
            let field_quotes: TokenStream2 = fields.named.iter().map(|field| {
                let field_name = &field.ident;
                let field_type = &field.ty;
                let field_type = quote! { #field_type }.to_string();

                if is_integer(field) {
                    quote! {  println!("Add integer {}!", stringify!(#field_name)); }
                }
                else {
                    quote! {  println!("Add field {} of type {}!", stringify!(#field_name), stringify!(#field_type)); }
                }
            }).collect();

            quote! {
                impl UI for #name {
                    fn create_viewer(&self, path: &str) {
                        println!("Create Viewer for {} with path '{}'!", stringify!(#name), path);
                        #field_quotes
                    }
                }
            }
        }
        _ => panic!("Expected a struct with named fields"),
    };

    gen.into()
}

fn is_integer(field: &Field) -> bool {
    matches!(&field.ty, Type::Path(type_path) if type_path.clone().into_token_stream().to_string() == "u32")
}
