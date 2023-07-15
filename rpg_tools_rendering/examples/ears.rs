extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::render_2_sets;
use rpg_tools_core::model::character::appearance::ear::{EarShape, Ears};
use rpg_tools_core::model::character::appearance::eye::{Eye, EyeShape, Eyes, PupilShape};
use rpg_tools_core::model::character::appearance::hair::Hair;
use rpg_tools_core::model::character::appearance::head::RealisticHeadShape::*;
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape, RealisticHeadShape};
use rpg_tools_core::model::character::appearance::mouth::Mouth;
use rpg_tools_core::model::character::appearance::skin::{Skin, SkinColor};
use rpg_tools_core::model::character::appearance::{Appearance, Size};
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;

pub mod utils;

fn main() {
    let shape_options = vec![EarShape::Pointed, EarShape::Round, EarShape::Square];
    let faces = vec![Oval, Rectangle, Round, Square, TriangleDown, TriangleUp];

    render_2_sets("ears.svg", shape_options, faces, create_appearance);
}

fn create_appearance(height: Length, shape: &EarShape, face: &RealisticHeadShape) -> Appearance {
    Appearance::head(
        Head {
            ears: Ears::Normal { shape: *shape },
            eyes: Eyes::Two {
                eye: Eye::Normal {
                    eye_shape: EyeShape::Circle,
                    pupil_shape: PupilShape::Circle,
                    pupil_color: Color::Gray,
                    background_color: Color::White,
                },
                distance: Size::Medium,
            },
            hair: Hair::None,
            mouth: Mouth::None,
            shape: HeadShape::Realistic(*face),
            skin: Skin::Skin(SkinColor::Light),
        },
        height,
    )
}
