extern crate macro_ui;
extern crate rpg_tools_core;

use macro_core::visitor::UI;
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_ui::io::write_each;
use rpg_tools_ui::ui::viewer::ViewerVisitor;

fn main() {
    println!("Generate tera code for viewer");

    let mut visitor = ViewerVisitor::new("appearance".to_string(), "    ".to_string());

    println!("Start visit");

    Appearance::visit(&mut visitor, "", false);

    println!("Finished visit");

    write_each("viewer.txt", visitor.get_lines()).expect("Couldn't write file!");
}
