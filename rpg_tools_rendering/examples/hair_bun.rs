extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::appearance::create_head_with_hair;
use crate::utils::render::{add_names, render_2_sets};
use rpg_tools_core::model::appearance::color::Color;
use rpg_tools_core::model::appearance::size::Size;
use rpg_tools_core::model::character::appearance::hair::bun::BunStyle;
use rpg_tools_core::model::character::appearance::hair::Hair;
use rpg_tools_core::model::character::appearance::head::HeadShape;

pub mod utils;

fn main() {
    let mut options = Vec::new();

    for style in BunStyle::get_all() {
        for size in Size::get_all() {
            options.push(create_bun(style, size));
        }
    }

    render_2_sets(
        "hair_bun.svg",
        options,
        add_names(HeadShape::get_all()),
        create_head_with_hair,
        true,
    );
}

fn create_bun(style: BunStyle, size: Size) -> (String, Hair) {
    (
        format!("{} + {}", style, size),
        Hair::Bun {
            style,
            size,
            hairline: Default::default(),
            color: Color::SaddleBrown,
        },
    )
}
