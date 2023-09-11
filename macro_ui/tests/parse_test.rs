use macro_convert::Convert;
use macro_core::parser::get_enum;
use macro_core::parser::{MockParser, UiParser};
use macro_core::visitor::{UiVisitor, UI};
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

fn assert_simple_enum(text: &str, value: SimpleEnum) {
    let parser = MockParser::new(HashMap::from([("test", text)]));

    assert_eq!(SimpleEnum::parse(&parser, "test", ""), value);
}
