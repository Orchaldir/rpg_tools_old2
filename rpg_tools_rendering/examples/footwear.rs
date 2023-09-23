extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::render_2_sets;
use rpg_tools_core::model::character::appearance::body::{Body, BodyShape};
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::equipment::appearance::footwear::{Footwear, FootwearStyle};
use rpg_tools_core::model::equipment::appearance::Clothing;
use rpg_tools_core::model::length::Length;

pub mod utils;

fn main() {
    let mut options = Vec::new();

    for style in FootwearStyle::get_all() {
        options.push(create(style));
    }

    render_2_sets(
        "footwear.svg",
        options,
        BodyShape::get_all(),
        create_appearance,
        false,
    );
}

fn create(style: FootwearStyle) -> Footwear {
    Footwear {
        color: Color::SaddleBrown,
        style,
        sole: Color::Gray,
    }
}

fn create_appearance(height: Length, footwear: &Footwear, shape: &BodyShape) -> Appearance {
    Appearance::humanoid(
        Body {
            shape: *shape,
            width: Default::default(),
            skin: Default::default(),
            clothing: Clothing::Simple {
                footwear: *footwear,
                pants: Default::default(),
                shirt: Default::default(),
            },
        },
        Default::default(),
        height,
    )
}
