extern crate macro_ui;
extern crate rpg_tools_core;

pub mod utils;

use crate::utils::write_each;
use macro_core::visitor::UI;
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::ui::editor::EditorVisitor;

fn main() {
    println!("Generate tera code for editor");

    let mut visitor = EditorVisitor::new("appearance".to_string(), "          ".to_string());

    println!("Start visit");

    Appearance::visit(&mut visitor, "", false);

    println!("Finished visit");

    write_each("editor.txt", visitor.get_lines()).expect("Couldn't write file!");
}
