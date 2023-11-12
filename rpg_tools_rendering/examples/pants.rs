extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::{add_names, render_2_sets};
use rpg_tools_core::model::appearance::color::Color;
use rpg_tools_core::model::appearance::length::Length;
use rpg_tools_core::model::character::appearance::body::{Body, BodyShape};
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::equipment::appearance::pants::{Pants, PantsStyle};
use rpg_tools_core::model::equipment::appearance::Clothing;

pub mod utils;

fn main() {
    let mut pants = vec![];

    for style in PantsStyle::get_all() {
        pants.push(create(style))
    }

    render_2_sets(
        "pants.svg",
        pants,
        add_names(BodyShape::get_all()),
        create_appearance,
        false,
    );
}

fn create(style: PantsStyle) -> (String, Pants) {
    (
        style.to_string(),
        Pants {
            style,
            color: Color::Blue,
            belt: None,
        },
    )
}

fn create_appearance(height: Length, pants: &Pants, shape: &BodyShape) -> Appearance {
    Appearance::humanoid(
        Body {
            shape: *shape,
            width: Default::default(),
            skin: Default::default(),
            clothing: Clothing::Simple {
                footwear: Default::default(),
                pants: *pants,
                shirt: Default::default(),
                outerwear: Default::default(),
            },
        },
        Default::default(),
        height,
    )
}
