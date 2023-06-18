extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::render_2_sets;
use rpg_tools_core::model::character::appearance::eye::{Eye, EyeShape, Eyes};
use rpg_tools_core::model::character::appearance::head::RealisticHeadShape::*;
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape, RealisticHeadShape};
use rpg_tools_core::model::character::appearance::mouth::Mouth;
use rpg_tools_core::model::character::appearance::skin::Skin;
use rpg_tools_core::model::character::appearance::{Appearance, Size};
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;

pub mod utils;

fn main() {
    let shape_options = vec![EyeShape::Almond, EyeShape::Ellipse, EyeShape::Circle];
    let faces = vec![Oval, Rectangle, Round, Square, TriangleDown, TriangleUp];

    render_2_sets("eye_shapes.svg", shape_options, faces, create_appearance);
}

fn create_appearance(height: Length, shape: &EyeShape, face: &RealisticHeadShape) -> Appearance {
    Appearance::head(
        Head {
            eyes: Eyes::Two {
                eye: Eye::Simple {
                    eye_shape: *shape,
                    color: Color::Yellow,
                },
                distance: Size::Medium,
            },
            mouth: Mouth::None,
            shape: HeadShape::Realistic(*face),
            skin: Skin::Scales(Color::Red),
        },
        height,
    )
}
