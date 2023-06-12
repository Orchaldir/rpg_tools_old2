extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::render_2_sets;
use rpg_tools_core::model::character::appearance::eye::{Eye, EyeDistance, EyeShape, Eyes};
use rpg_tools_core::model::character::appearance::head::RealisticHeadShape::*;
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape, RealisticHeadShape};
use rpg_tools_core::model::character::appearance::skin::Skin;
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;

pub mod utils;

fn main() {
    let eye = Eye::Simple {
        eye_shape: EyeShape::Round,
        color: Color::Yellow,
    };
    let eyes_options = vec![
        Eyes::One(eye),
        Eyes::Two {
            eye,
            distance: EyeDistance::Low,
        },
        Eyes::Two {
            eye,
            distance: EyeDistance::Medium,
        },
        Eyes::Two {
            eye,
            distance: EyeDistance::High,
        },
    ];
    let faces = vec![Oval, Rectangle, Round, Square, TriangleDown, TriangleUp];

    render_2_sets("eyes.svg", eyes_options, faces, create_appearance);
}

fn create_appearance(height: Length, eyes: &Eyes, face: &RealisticHeadShape) -> Appearance {
    Appearance::head(
        Head {
            eyes: eyes.clone(),
            shape: HeadShape::Realistic(*face),
            skin: Skin::Scales(Color::Red),
        },
        height,
    )
}
