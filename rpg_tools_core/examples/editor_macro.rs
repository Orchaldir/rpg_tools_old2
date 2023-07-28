extern crate rpg_tools_core;
extern crate ui_macro;

pub mod util;

use crate::util::write_each;
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::ui::editor::EditorVisitor;
use rpg_tools_core::ui::{UiVisitor, UI};
use ui_macro::ui;

fn main() {
    println!("Generate tera code for editor");

    let mut visitor = EditorVisitor::new("appearance".to_string(), "          ".to_string());

    println!("Start visit");

    Appearance::create_viewer(&mut visitor, "start", "");

    println!("Finished visit");

    write_each("editor.txt", visitor.get_lines()).expect("Couldn't write file!");
}
