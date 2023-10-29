extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::{add_names, render_2_sets};
use rpg_tools_core::model::character::appearance::eye::brow::EyeBrows;
use rpg_tools_core::model::character::appearance::eye::shape::EyeShape;
use rpg_tools_core::model::character::appearance::eye::{Eye, Eyes};
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape};
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;
use rpg_tools_core::model::size::Size;

pub mod utils;

fn main() {
    let eye = Eye::Simple {
        eye_shape: EyeShape::Circle,
        color: Color::Yellow,
    };
    let eyes_options = vec![
        (
            "One".to_string(),
            Eyes::One {
                eye,
                eyebrows: EyeBrows::None,
            },
        ),
        create_two(eye, Size::Small),
        create_two(eye, Size::Medium),
        create_two(eye, Size::Large),
    ];
    let faces = add_names(HeadShape::get_all());

    render_2_sets("eyes.svg", eyes_options, faces, create_appearance, false);
}

fn create_two(eye: Eye, distance: Size) -> (String, Eyes) {
    (
        format!("Two + {}", distance),
        Eyes::Two {
            eye,
            eyebrows: EyeBrows::None,
            distance,
            eyewear: Default::default(),
        },
    )
}

fn create_appearance(height: Length, eyes: &Eyes, face: &HeadShape) -> Appearance {
    Appearance::head(
        Head {
            ears: Default::default(),
            eyes: *eyes,
            hair: Default::default(),
            mouth: Default::default(),
            shape: *face,
            skin: Default::default(),
        },
        height,
    )
}
