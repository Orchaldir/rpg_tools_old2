extern crate macro_convert;
extern crate rpg_tools_core;

use macro_ui::ui;

#[derive(ui, Debug, Default)]
pub struct Test {
    a: u32,
    b: u32,
}

fn main() {
    let test = Test::default();
    println!("Values:{:?}", test);
}
