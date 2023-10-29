extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::{add_names, render_2_sets};
use rpg_tools_core::model::character::appearance::eye::shape::EyeShape;
use rpg_tools_core::model::character::appearance::eye::{Eye, Eyes};
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape};
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;

pub mod utils;

fn main() {
    render_2_sets(
        "eye_shapes.svg",
        add_names(EyeShape::get_all()),
        add_names(HeadShape::get_all()),
        create_appearance,
        false,
    );
}

fn create_appearance(height: Length, shape: &EyeShape, face: &HeadShape) -> Appearance {
    Appearance::head(
        Head {
            ears: Default::default(),
            eyes: Eyes::Two {
                eye: Eye::Simple {
                    eye_shape: *shape,
                    color: Color::Yellow,
                },
                eyebrows: Default::default(),
                distance: Default::default(),
                eyewear: Default::default(),
            },
            hair: Default::default(),
            mouth: Default::default(),
            shape: *face,
            skin: Default::default(),
        },
        height,
    )
}
