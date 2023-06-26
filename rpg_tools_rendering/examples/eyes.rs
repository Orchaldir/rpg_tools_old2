extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::render_2_sets;
use rpg_tools_core::model::character::appearance::eye::{Eye, EyeShape, Eyes};
use rpg_tools_core::model::character::appearance::hair::Hair;
use rpg_tools_core::model::character::appearance::head::RealisticHeadShape::*;
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape, RealisticHeadShape};
use rpg_tools_core::model::character::appearance::mouth::Mouth;
use rpg_tools_core::model::character::appearance::skin::Skin;
use rpg_tools_core::model::character::appearance::{Appearance, Size};
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;

pub mod utils;

fn main() {
    let eye = Eye::Simple {
        eye_shape: EyeShape::Circle,
        color: Color::Yellow,
    };
    let eyes_options = vec![
        Eyes::One(eye),
        Eyes::Two {
            eye,
            distance: Size::Low,
        },
        Eyes::Two {
            eye,
            distance: Size::Medium,
        },
        Eyes::Two {
            eye,
            distance: Size::High,
        },
    ];
    let faces = vec![Oval, Rectangle, Round, Square, TriangleDown, TriangleUp];

    render_2_sets("eyes.svg", eyes_options, faces, create_appearance);
}

fn create_appearance(height: Length, eyes: &Eyes, face: &RealisticHeadShape) -> Appearance {
    Appearance::head(
        Head {
            eyes: eyes.clone(),
            hair: Hair::None,
            mouth: Mouth::None,
            shape: HeadShape::Realistic(*face),
            skin: Skin::Scales(Color::Red),
        },
        height,
    )
}
