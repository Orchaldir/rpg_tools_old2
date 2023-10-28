extern crate macro_ui;
extern crate rpg_tools_core;

pub mod utils;

use crate::utils::write_each;
use macro_core::visitor::{visit_option, UI};
use macro_ui::ui;
use rpg_tools_core::model::size::Size;
use rpg_tools_core::ui::editor::EditorVisitor;
use serde::{Deserialize, Serialize};

#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Test {
    pub option: Option<Size>,
}

fn main() {
    println!("Generate tera code for editor");

    let mut visitor = EditorVisitor::new("appearance".to_string(), "          ".to_string());

    println!("Start visit");

    Test::visit(&mut visitor, "", false);

    println!("Finished visit");

    write_each("editor.txt", visitor.get_lines()).expect("Couldn't write file!");
}
