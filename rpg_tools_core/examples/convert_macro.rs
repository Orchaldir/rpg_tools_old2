extern crate macro_convert;
extern crate rpg_tools_core;

use macro_convert::Convert;

#[derive(Convert, Default)]
pub enum Test {
    A,
    #[default]
    B,
    C,
}

fn main() {
    println!("Values:{:?}", Test::get_all());
}