extern crate rpg_tools_core;
extern crate ui_macro;

pub mod util;

use crate::util::write_each;
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::ui::editor::EditorVisitor;
use rpg_tools_core::ui::{UiVisitor, UI};
use ui_macro::ui;

#[derive(ui)]
pub enum EnumTest {
    VALUE,
    NAME,
}

#[derive(ui)]
pub enum TupleEnum {
    T(u32),
    N,
}

#[derive(ui)]
pub struct InnerStruct {
    c: u32,
    d: u32,
    tuple: TupleEnum,
}

#[derive(ui)]
pub struct Test {
    a: u32,
    b: u32,
    simple: EnumTest,
    inner: InnerStruct,
}

fn main() {
    println!("Generate tera code for editor");

    let mut visitor = EditorVisitor::new("appearance".to_string(), "".to_string());

    println!("Start visit");

    Test::create_viewer(&mut visitor, "start", "");

    println!("Finished visit");

    write_each("editor.txt", visitor.get_lines()).expect("Couldn't write file!");
}
