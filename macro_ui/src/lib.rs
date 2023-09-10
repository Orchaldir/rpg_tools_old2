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
            impl UI for #name {
                fn create_viewer(visitor: &mut dyn UiVisitor, spaces: &str, _in_tuple: bool) {
                    println!("{}Add simple enum {} with path '{}'!", spaces, stringify!(#name), visitor.get_path());
                    visitor.add_simple_enum(&[#(stringify!(#variants).to_string()),*]);
                }
            }

            use rpg_tools_core::ui::parser::get_enum;

            #[automatically_derived]
            impl #name {
                fn parse(parser: &dyn UiParser, path: &str, spaces: &str) -> #name {
                    println!("{}Parse simple enum {} with path '{}'", spaces, stringify!(#name), path);
                    get_enum(parser, path)
                }
            }
        };
    }

    let field_quotes = visit_enum_variants(data);

    quote! {
        #[automatically_derived]
        impl UI for #name {
            fn create_viewer(visitor: &mut dyn UiVisitor, spaces: &str, _in_tuple: bool) {
                println!("{}Create Viewer for enum {} with path '{}'!", spaces, stringify!(#name), visitor.get_path());
                visitor.enter_enum(&[#(stringify!(#variants).to_string()),*]);
                let inner_spaces = format!("  {}", spaces);
                #field_quotes
                visitor.leave_enum();
                println!("{}Finish Viewer for enum {} with path '{}'!", spaces, stringify!(#name), visitor.get_path());
            }
        }

        #[automatically_derived]
        impl #name {
            fn parse(parser: &dyn UiParser, path: &str, spaces: &str) -> #name {
                println!("{}Parse complex enum {} with path '{}'", spaces, stringify!(#name), path);
                println!("{}type '{}'", spaces, parser.get_str(&format!("{}.type", path)).unwrap_or(""));

                //match parser.get_str(&format!("{}.type", path)) {

                //}
                #name::default()
            }
        }
    }
}

fn visit_enum_variants(data: &DataEnum) -> TokenStream2 {
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

fn handle_struct(name: &Ident, fields: &FieldsNamed) -> TokenStream2 {
    let visited_fields: TokenStream2 = fields.named.iter().map(visit_struct_field).collect();
    let parsed_fields: TokenStream2 = fields.named.iter().map(parse_struct_field).collect();

    quote! {
        #[automatically_derived]
        impl UI for #name {
            fn create_viewer(visitor: &mut dyn UiVisitor, spaces: &str, in_tuple: bool) {
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
            fn parse(parser: &dyn UiParser, path: &str, spaces: &str) -> #name {
                println!("{}Parse struct {} with path '{}'", spaces, stringify!(#name), path);
                Self {
                    #parsed_fields
                }
            }
        }
    }
}

fn visit_struct_field(field: &Field) -> TokenStream2 {
    let field_name = &field.ident;

    if is_integer(field) {
        quote! {
            println!("{}Add integer '{}'!", &inner_spaces, stringify!(#field_name));
            visitor.add_integer(stringify!(#field_name));
        }
    } else {
        let name = &get_field_type(field);
        quote! {
            visitor.enter_child(stringify!(#field_name));
            #name::create_viewer(visitor, &inner_spaces, false);
            visitor.leave_child();
        }
    }
}

fn parse_struct_field(field: &Field) -> TokenStream2 {
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
            #name::create_viewer(visitor, &inner_spaces, true);
            visitor.leave_child();
        }
    }
}

fn get_field_type(field: &Field) -> Option<Ident> {
    match &field.ty {
        Type::Path(type_path) => type_path.path.segments.first().map(|s| s.ident.clone()),
        _ => None,
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
