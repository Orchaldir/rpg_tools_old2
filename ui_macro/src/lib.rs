use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(ui)]
pub fn ui_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_ui_macro(&ast)
}

fn impl_ui_macro(ast: &syn::DeriveInput) -> TokenStream {
    println!("ast={:?}", ast);

    let name = &ast.ident;

    println!("name={:?}", name);

    let gen = quote! {
        impl UI for #name {
            fn create_viewer(&self, path: &str) {
                println!("Create Viewer for {} with path '{}'!", stringify!(#name), path);
            }
        }
    };
    gen.into()
}
