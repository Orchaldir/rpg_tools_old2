use quote::ToTokens;
use syn::{DataEnum, Field, Fields, GenericArgument, Ident, PathArguments, Type};

pub fn get_field_type(field: &Field) -> Option<Ident> {
    match &field.ty {
        Type::Path(type_path) => type_path.path.segments.first().map(|s| s.ident.clone()),
        _ => None,
    }
}

pub fn is_integer(field: &Field) -> bool {
    matches!(&field.ty, Type::Path(type_path) if type_path.clone().into_token_stream().to_string() == "u32")
}

pub fn is_option(field: &Field) -> bool {
    matches!(&field.ty, Type::Path(type_path) if type_path.clone().into_token_stream().to_string().starts_with("Option"))
}

pub fn is_option_u32(field: &Field) -> bool {
    match &field.ty {
        Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.first() {
                return match &segment.arguments {
                    PathArguments::AngleBracketed(args) => {
                        if let Some(GenericArgument::Type(t)) = args.args.first() {
                            return matches!(t, Type::Path(type_path) if type_path.clone().into_token_stream().to_string() == "u32");
                        }
                        false
                    }
                    _ => false,
                };
            }
            false
        }
        _ => false,
    }
}

pub fn is_simple_enum(data: &DataEnum) -> bool {
    for variant in &data.variants {
        if variant.fields != Fields::Unit {
            return false;
        }
    }

    true
}
