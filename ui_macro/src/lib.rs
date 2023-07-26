use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::{Data, DataEnum, DataStruct, Field, Fields, FieldsNamed, Ident, Type};

#[proc_macro_derive(ui)]
pub fn ui_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
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
    if is_simple_enum(data) {
        let variants: Vec<Ident> = data.variants.iter().map(|v| v.ident.clone()).collect();
        let variants = quote! { #(#variants)* };
        return quote! {
            impl UI for #name {
                fn create_viewer(&self, visitor: &mut dyn UiVisitor, path: &str, spaces: &str) {
                    println!("{}Add simple enum {} with path '{}' & variants '{}'!", spaces, stringify!(#name), path, stringify!(#variants));
                }
            }
        };
    } else if is_tuple_enum(data) {
        let field_quotes = handle_enum_variants(data);

        return quote! {
            impl UI for #name {
                fn create_viewer(&self, visitor: &mut dyn UiVisitor, path: &str, spaces: &str) {
                    println!("{}Create Viewer for tuple enum {} with path '{}'!", spaces, stringify!(#name), path);
                    let inner_spaces = format!("  {}", spaces);
                    #field_quotes
                    println!("{}Finish Viewer for tuple enum {} with path '{}'!", spaces, stringify!(#name), path);
                }
            }
        };
    }

    let field_quotes = handle_enum_variants(data);

    quote! {
        impl UI for #name {
            fn create_viewer(&self, visitor: &mut dyn UiVisitor, path: &str, spaces: &str) {
                println!("{}Create Viewer for enum {} with path '{}'!", spaces, stringify!(#name), path);
                let inner_spaces = format!("  {}", spaces);
                #field_quotes
                println!("{}Finish Viewer for enum {} with path '{}'!", spaces, stringify!(#name), path);
            }
        }
    }
}

fn handle_enum_variants(data: &DataEnum) -> TokenStream2 {
    let mut results: Vec<TokenStream2> = Vec::new();

    for variant in &data.variants {
        let variant_name = &variant.ident;

        match &variant.fields {
            Fields::Named(fields) => {
                results.push(quote! {  println!("{}Add named variant '{}'!", &inner_spaces, stringify!(#variant_name)); });

                for field in &fields.named {
                    results.push(handle_field(field));
                }
            }
            Fields::Unnamed(fields) => {
                if fields.unnamed.len() != 1 {
                    panic!("Tuple enums are only supported with 1 field!")
                }

                results.push(quote! {  println!("{}Add unnamed variant '{}'!", &inner_spaces, stringify!(#variant_name)); });

                results.push(handle_field_name(&fields.unnamed[0], "c"));
            }
            Fields::Unit => {
                results.push(quote! {  println!("{}Add simple variant '{}'!", &inner_spaces, stringify!(#variant_name)); });
            }
        }
    }

    quote! { #(#results)* }
}

fn handle_struct(name: &Ident, fields: &FieldsNamed) -> TokenStream2 {
    let field_quotes: TokenStream2 = fields.named.iter().map(handle_field).collect();

    quote! {
        impl UI for #name {
            fn create_viewer(&self, visitor: &mut dyn UiVisitor, path: &str, spaces: &str) {
                println!("{}Create Viewer for struct {} with path '{}'!", spaces, stringify!(#name), path);
                visitor.enter_struct();
                let inner_spaces = format!("  {}", spaces);
                #field_quotes
                visitor.leave_struct();
                println!("{}Finish Viewer for struct {} with path '{}'!", spaces, stringify!(#name), path);
            }
        }
    }
}

fn handle_field(field: &Field) -> TokenStream2 {
    let field_name = &field.ident;

    if is_integer(field) {
        quote! {  println!("{}Add integer '{}'!", &inner_spaces, stringify!(#field_name)); }
    } else {
        quote! {  self.#field_name.create_viewer(visitor, &format!("{}.{}", path, stringify!(#field_name)), &inner_spaces); }
    }
}

fn handle_field_name(field: &Field, field_name: &str) -> TokenStream2 {
    if is_integer(field) {
        quote! {  println!("{}Add integer '{}'!", &inner_spaces,#field_name); }
    } else {
        quote! {  self.#field_name.create_viewer(visitor, &format!("{}.{}", path,#field_name), &inner_spaces); }
    }
}

fn is_integer(field: &Field) -> bool {
    matches!(&field.ty, Type::Path(type_path) if type_path.clone().into_token_stream().to_string() == "u32")
}

fn is_simple_enum(data: &DataEnum) -> bool {
    for variant in &data.variants {
        if variant.fields != Fields::Unit {
            return false;
        }
    }

    true
}

fn is_tuple_enum(data: &DataEnum) -> bool {
    for variant in &data.variants {
        if let Fields::Unnamed(..) = variant.fields {
            return true;
        }
    }

    false
}
