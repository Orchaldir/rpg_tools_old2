extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::{add_names, render_2_sets};
use rpg_tools_core::model::appearance::color::Color;
use rpg_tools_core::model::appearance::length::Length;
use rpg_tools_core::model::character::appearance::body::{Body, BodyShape};
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::equipment::appearance::outerwear::cloak::Cloak;
use rpg_tools_core::model::equipment::appearance::outerwear::{Outerwear, OuterwearLength};
use rpg_tools_core::model::equipment::appearance::Clothing;

pub mod utils;

fn main() {
    render_2_sets(
        "cloaks.svg",
        add_names(OuterwearLength::get_all()),
        add_names(BodyShape::get_all()),
        create_appearance,
        true,
    );
}

fn create_appearance(height: Length, length: &OuterwearLength, shape: &BodyShape) -> Appearance {
    let cloak = Cloak {
        length: *length,
        outer_color: Color::Navy,
        inner_color: Color::Blue,
    };
    Appearance::humanoid(
        Body {
            shape: *shape,
            width: Default::default(),
            skin: Default::default(),
            clothing: Clothing::Simple {
                footwear: Default::default(),
                pants: Default::default(),
                shirt: Default::default(),
                outerwear: Outerwear::Cloak(cloak),
            },
        },
        Default::default(),
        height,
    )
}
