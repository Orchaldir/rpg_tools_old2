extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::appearance::create_humanoid_with_hair;
use crate::utils::render::render_2_sets;
use rpg_tools_core::model::character::appearance::hair::long::LongHairStyle;
use rpg_tools_core::model::character::appearance::hair::Hair;
use rpg_tools_core::model::character::appearance::head::HeadShape;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;

pub mod utils;

fn main() {
    let mut options = Vec::new();

    for style in LongHairStyle::get_all() {
        for l in 1..6 {
            options.push(create_long(style, l as f32 * 0.2));
        }
    }

    render_2_sets(
        "hair_long.svg",
        options,
        HeadShape::get_all(),
        create_humanoid_with_hair,
        true,
    );
}

fn create_long(style: LongHairStyle, length: f32) -> Hair {
    Hair::Long {
        style,
        hairline: Default::default(),
        length: Length::from_metre(length),
        color: Color::SaddleBrown,
    }
}
