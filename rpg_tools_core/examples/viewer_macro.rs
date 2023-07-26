extern crate rpg_tools_core;
extern crate ui_macro;

use rpg_tools_core::ui::{UiVisitor, ViewerVisitor, UI};
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
struct OuterStruct {
    number_d: u32,
    //inner: InnerStruct,
    simple: SimpleEnum,
    number_e: u32,
}

fn main() {
    println!("Generate tera code for viewer");

    let data = OuterStruct {
        /*
        inner: InnerStruct {
            number_a: 0,
            simple: SimpleEnum::B,
            tuple: TupleEnum::D(99),
            data: StructEnum::DATA { one: 4, two: 5 },
            number_c: 0,
        },
        */
        simple: SimpleEnum::B,
        number_d: 0,
        number_e: 0,
    };

    let mut visitor = ViewerVisitor::new("test".to_string(), "".to_string());

    println!("Start visit");

    data.create_viewer(&mut visitor, "start", "");

    println!("Finished visit");

    visitor.get_lines().iter().for_each(|l| println!("{}", l))
}
