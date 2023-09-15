use macro_convert::Convert;
use macro_core::parser::get_enum;
use macro_core::parser::MockParser;
use macro_ui::ui;
use std::collections::HashMap;

#[derive(Convert, ui, Debug, Default, PartialEq)]
pub enum SimpleEnum {
    #[default]
    A,
    B,
}

#[derive(ui, Debug, Default, PartialEq)]
pub struct SimpleStruct {
    pub c: u32,
}

#[derive(ui, Debug, Default, PartialEq)]
pub enum ComplexEnum {
    #[default]
    C,
    D(SimpleStruct),
    E {
        d: u32,
        e: u32,
    },
}

#[derive(ui, Debug, Default, PartialEq)]
pub struct ComplexStruct {
    pub a: u32,
    pub b: u32,
    pub inner: SimpleStruct,
    pub simple: SimpleEnum,
    pub complex: ComplexEnum,
}

#[test]
fn test_simple_enum() {
    assert_simple_enum("A", SimpleEnum::A);
    assert_simple_enum("B", SimpleEnum::B);
}

#[test]
fn test_simple_struct() {
    let parser = MockParser::new(HashMap::from([("s.c", "42")]));

    assert_eq!(
        SimpleStruct::parse(&parser, "s", ""),
        SimpleStruct { c: 42 }
    );
}

#[test]
fn test_complex_enum_with_simple_variant() {
    assert_complex_enum(HashMap::from([("complex.type", "C")]), ComplexEnum::C)
}

#[test]
fn test_complex_enum_with_struct() {
    assert_complex_enum(
        HashMap::from([("complex.type", "D"), ("complex.c", "99")]),
        ComplexEnum::D(SimpleStruct { c: 99 }),
    )
}

#[test]
fn test_complex_enum_with_fields() {
    assert_complex_enum(
        HashMap::from([
            ("complex.type", "E"),
            ("complex.d", "10"),
            ("complex.e", "20"),
        ]),
        ComplexEnum::E { d: 10, e: 20 },
    )
}

#[test]
fn test_complex_struct() {
    let parser = MockParser::new(HashMap::from([
        ("test.a", "2"),
        ("test.b", "3"),
        ("test.inner.c", "4"),
        ("test.simple", "B"),
        ("test.complex.type", "E"),
        ("test.complex.d", "6"),
        ("test.complex.e", "7"),
    ]));

    assert_eq!(
        ComplexStruct::parse(&parser, "test", ""),
        ComplexStruct {
            a: 2,
            b: 3,
            inner: SimpleStruct { c: 4 },
            simple: SimpleEnum::B,
            complex: ComplexEnum::E { d: 6, e: 7 },
        }
    );
}

fn assert_simple_enum(text: &str, value: SimpleEnum) {
    let parser = MockParser::new(HashMap::from([("test", text)]));

    assert_eq!(SimpleEnum::parse(&parser, "test", ""), value);
}

fn assert_complex_enum(data: HashMap<&str, &str>, value: ComplexEnum) {
    let parser = MockParser::new(data);

    assert_eq!(ComplexEnum::parse(&parser, "complex", ""), value);
}
