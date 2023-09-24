extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::{add_names, render_2_sets};
use rpg_tools_core::model::character::appearance::eye::pupil::PupilShape;
use rpg_tools_core::model::character::appearance::eye::shape::EyeShape;
use rpg_tools_core::model::character::appearance::eye::{Eye, Eyes};
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape};
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;

pub mod utils;

fn main() {
    render_2_sets(
        "pupil_shapes.svg",
        add_names(PupilShape::get_all()),
        add_names(EyeShape::get_all()),
        create_appearance,
        false,
    );
}

fn create_appearance(height: Length, pupil_shape: &PupilShape, eye_shape: &EyeShape) -> Appearance {
    Appearance::head(
        Head {
            ears: Default::default(),
            eyes: Eyes::Two {
                eye: Eye::Normal {
                    eye_shape: *eye_shape,
                    pupil_shape: *pupil_shape,
                    pupil_color: Color::Black,
                    background_color: Color::Yellow,
                },
                eyebrows: Default::default(),
                distance: Default::default(),
            },
            hair: Default::default(),
            mouth: Default::default(),
            shape: HeadShape::Oval,
            skin: Default::default(),
        },
        height,
    )
}
