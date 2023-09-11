extern crate macro_ui;
extern crate rpg_tools_core;

pub mod utils;

use crate::utils::write_each;
use macro_core::visitor::UI;
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::ui::viewer::ViewerVisitor;

fn main() {
    println!("Generate tera code for viewer");

    let mut visitor = ViewerVisitor::new("appearance".to_string(), "    ".to_string());

    println!("Start visit");

    Appearance::create_viewer(&mut visitor, "", false);

    println!("Finished visit");

    write_each("viewer.txt", visitor.get_lines()).expect("Couldn't write file!");
}
