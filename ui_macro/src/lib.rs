use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::{Data, DataEnum, DataStruct, Field, Fields, FieldsNamed, Ident, Type};

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
        }) => handle_struct(name, fields),
        Data::Enum(data) => handle_enum(name, data),
        _ => panic!("Expected a struct with named fields"),
    };

    gen.into()
}

fn handle_enum(name: &Ident, data: &DataEnum) -> TokenStream2 {
    quote! {
        impl UI for #name {
            fn create_viewer(&self, path: &str, spaces: &str) {
                println!("{}Create Viewer for enum {} with path '{}'!", spaces, stringify!(#name), path);
                let inner_spaces = format!("  {}", spaces);
                println!("{}Finish Viewer for enum {} with path '{}'!", spaces, stringify!(#name), path);
            }
        }
    }
}

fn handle_struct(name: &Ident, fields: &FieldsNamed) -> TokenStream2 {
    let field_quotes: TokenStream2 = fields.named.iter().map(|field| {
        let field_name = &field.ident;

        if is_integer(field) {
            quote! {  println!("{}Add integer {}!", &inner_spaces, stringify!(#field_name)); }
        } else {
            quote! {  self.#field_name.create_viewer(&format!("{}.{}", path, stringify!(#field_name)), &inner_spaces); }
        }
    }).collect();

    quote! {
        impl UI for #name {
            fn create_viewer(&self, path: &str, spaces: &str) {
                println!("{}Create Viewer for struct {} with path '{}'!", spaces, stringify!(#name), path);
                let inner_spaces = format!("  {}", spaces);
                #field_quotes
                println!("{}Finish Viewer for struct {} with path '{}'!", spaces, stringify!(#name), path);
            }
        }
    }
}

fn is_integer(field: &Field) -> bool {
    matches!(&field.ty, Type::Path(type_path) if type_path.clone().into_token_stream().to_string() == "u32")
}
