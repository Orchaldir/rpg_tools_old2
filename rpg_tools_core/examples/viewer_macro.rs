extern crate rpg_tools_core;
extern crate ui_macro;

use ui_macro::UI;

#[derive(UI)]
struct Test {
    number_a: u32,
    number_b: u32,
    number_c: u32,
}

fn main() {
    println!("Generate tera code for viewer");
}
