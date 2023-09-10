use quote::ToTokens;
use syn::{DataEnum, Field, Fields, Ident, Type};

pub fn get_field_type(field: &Field) -> Option<Ident> {
    match &field.ty {
        Type::Path(type_path) => type_path.path.segments.first().map(|s| s.ident.clone()),
        _ => None,
    }
}

pub fn is_integer(field: &Field) -> bool {
    matches!(&field.ty, Type::Path(type_path) if type_path.clone().into_token_stream().to_string() == "u32")
}

pub fn is_simple_enum(data: &DataEnum) -> bool {
    for variant in &data.variants {
        if variant.fields != Fields::Unit {
            return false;
        }
    }

    true
}

pub fn is_tuple_enum(data: &DataEnum) -> bool {
    for variant in &data.variants {
        if let Fields::Unnamed(..) = variant.fields {
            return true;
        }
    }

    false
}
