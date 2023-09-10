extern crate macro_convert;
extern crate rpg_tools_core;

use rpg_tools_core::ui::parser::{MockParser, UiParser};
use rpg_tools_core::ui::{UiVisitor, UI};
use std::collections::HashMap;

use macro_ui::ui;

#[derive(ui, Debug, Default)]
pub struct Test {
    pub a: u32,
    pub b: u32,
}

impl Test {
    pub fn new(a: u32, b: u32) -> Self {
        Self { a, b }
    }
}

fn main() {
    let parser = MockParser::new(HashMap::from([("test.a", "2"), ("test.b", "3")]));
    let test = Test::parse(&parser, "test", "");
    println!("Values:{:?}", test);
}
