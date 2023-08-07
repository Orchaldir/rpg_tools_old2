extern crate macro_convert;
extern crate rpg_tools_core;

use macro_convert::Convert;

#[derive(Convert)]
pub enum Test {
    A,
    B,
    C,
}

fn main() {
    println!("Generate tera code for viewer");
}
