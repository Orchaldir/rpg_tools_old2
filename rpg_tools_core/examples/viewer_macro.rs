extern crate rpg_tools_core;
extern crate ui_macro;

use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::ui::viewer::ViewerVisitor;
use rpg_tools_core::ui::UI;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    println!("Generate tera code for viewer");

    let mut visitor = ViewerVisitor::new("appearance".to_string(), "    ".to_string());

    println!("Start visit");

    Appearance::create_viewer(&mut visitor, "start", "");

    println!("Finished visit");

    write_each("viewer.txt", visitor.get_lines()).expect("Couldn't write file!");
}

fn write_each(
    path: impl AsRef<Path>,
    items: impl IntoIterator<Item = impl AsRef<[u8]>>,
) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    for i in items {
        file.write_all(i.as_ref())?;
        file.write_all("\n".as_ref())?;
    }
    file.sync_all()
}
