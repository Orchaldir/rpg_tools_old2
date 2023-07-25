extern crate rpg_tools_core;
extern crate ui_macro;

use rpg_tools_core::ui::UI;
use ui_macro::ui;

#[derive(ui)]
struct InnerStruct {
    number_a: u32,
    number_b: u32,
    number_c: u32,
}

#[derive(ui)]
struct OuterStruct {
    inner: InnerStruct,
    number_d: u32,
    number_e: u32,
}

fn main() {
    println!("Generate tera code for viewer");

    let viewer = OuterStruct {
        inner: InnerStruct {
            number_a: 0,
            number_b: 0,
            number_c: 0,
        },
        number_d: 0,
        number_e: 0,
    };

    viewer.create_viewer("start");
}
