extern crate macro_convert;
extern crate rpg_tools_core;

use macro_convert::Convert;
use macro_core::parser::get_enum;
use macro_core::parser::{MockParser, UiParser};
use macro_core::visitor::{UiVisitor, UI};
use std::collections::HashMap;

use macro_ui::ui;

#[derive(Convert, ui, Debug, Default)]
pub enum SimpleEnum {
    #[default]
    A,
    B,
}

#[derive(ui, Debug, Default)]
pub struct Inner {
    pub c: u32,
}

#[derive(ui, Debug, Default)]
pub enum ComplexEnum {
    #[default]
    C,
    D(Inner),
    E {
        d: u32,
        e: u32,
    },
}

#[derive(ui, Debug, Default)]
pub struct Test {
    pub a: u32,
    pub b: u32,
    pub inner: Inner,
    pub simple: SimpleEnum,
    pub complex: ComplexEnum,
}

fn main() {
    let parser = MockParser::new(HashMap::from([
        ("test.a", "2"),
        ("test.b", "3"),
        ("test.inner.c", "4"),
        ("test.simple", "B"),
        ("test.complex.type", "E"),
        ("test.complex.d", "6"),
        ("test.complex.e", "7"),
    ]));
    let test = Test::parse(&parser, "test", "");
    println!("Values:{:?}", test);
}
