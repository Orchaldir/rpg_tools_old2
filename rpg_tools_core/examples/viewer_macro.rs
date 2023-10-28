extern crate macro_ui;
extern crate rpg_tools_core;

pub mod utils;

use crate::utils::write_each;
use macro_core::visitor::{UI, visit_option};
use macro_ui::ui;
use rpg_tools_core::model::size::Size;
use rpg_tools_core::ui::viewer::ViewerVisitor;
use serde::{Deserialize, Serialize};

#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Test {
    pub option: Option<Size>,
}

fn main() {
    println!("Generate tera code for viewer");

    let mut visitor = ViewerVisitor::new("appearance".to_string(), "    ".to_string());

    println!("Start visit");

    Test::visit(&mut visitor, "", false);

    println!("Finished visit");

    write_each("viewer.txt", visitor.get_lines()).expect("Couldn't write file!");
}
