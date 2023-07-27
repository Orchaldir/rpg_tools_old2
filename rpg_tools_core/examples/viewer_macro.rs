extern crate rpg_tools_core;
extern crate ui_macro;

use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::ui::{UiVisitor, ViewerVisitor, UI};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use ui_macro::ui;

#[derive(ui)]
enum SimpleEnum {
    A,
    B,
    C,
}

#[derive(ui)]
enum TupleEnum {
    D(u32),
    E,
    F(u32),
}

#[derive(ui)]
enum StructEnum {
    DATA { one: u32, two: u32 },
    STRUCT { three: u32, second: SecondStruct },
    NONE,
}

#[derive(ui)]
struct InnerStruct {
    number_a: u32,
    simple: SimpleEnum,
    tuple: TupleEnum,
    data: StructEnum,
    number_c: u32,
}

#[derive(ui)]
struct SecondStruct {
    a: u32,
    b: u32,
}

#[derive(ui)]
struct OuterStruct {
    number_d: u32,
    tuple: TupleEnum,
    data: StructEnum,
    number_e: u32,
}

fn main() {
    println!("Generate tera code for viewer");

    let mut visitor = ViewerVisitor::new("appearance".to_string(), "    ".to_string());

    println!("Start visit");

    Appearance::create_viewer(&mut visitor, "start", "");

    println!("Finished visit");

    write_each("test.txt", visitor.get_lines());
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
