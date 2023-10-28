use quote::ToTokens;
use syn::PathArguments::AngleBracketed;
use syn::{DataEnum, Field, Fields, GenericArgument, Ident, Type};

pub fn get_field_type(field: &Field) -> Option<Ident> {
    match &field.ty {
        Type::Path(type_path) => type_path.path.segments.first().map(|s| s.ident.clone()),
        _ => None,
    }
}

pub fn get_option_type(field: &Field) -> Option<Ident> {
    if let Some(Type::Path(type_path)) = &field.ty {
        if let Some(segment) = type_path.path.segments.first() {
            if let Some(AngleBracketed(args)) = &segment.arguments {
                if let Some(GenericArgument::Type(Type::Path(type_path))) = args.args.first() {
                    return type_path.path.segments.first().map(|s| s.ident.clone());
                }
            }
        }
    }
    None
}

pub fn is_integer(field: &Field) -> bool {
    is_type(&field.ty, "u32")
}

pub fn is_option(field: &Field) -> bool {
    matches!(&field.ty, Type::Path(type_path) if type_path.clone().into_token_stream().to_string().starts_with("Option"))
}

pub fn is_simple_enum(data: &DataEnum) -> bool {
    for variant in &data.variants {
        if variant.fields != Fields::Unit {
            return false;
        }
    }

    true
}

fn is_type(t: &Type, desired: &str) -> bool {
    matches!(t, Type::Path(type_path) if type_path.clone().into_token_stream().to_string() == desired)
}
