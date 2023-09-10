extern crate macro_convert;
extern crate rpg_tools_core;
use rpg_tools_core::ui::parser::UiParser;
use rpg_tools_core::ui::{UiVisitor, UI};

use macro_ui::ui;

#[derive(ui, Debug, Default)]
pub struct Test {
    a: u32,
    b: u32,
}

impl Test {
    pub fn new(a: u32, b: u32) -> Self {
        Self { a, b }
    }
}

fn main() {
    let test = Test::new(1, 2);
    println!("Values:{:?}", test);
}
