extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::render_2_sets;
use rpg_tools_core::model::character::appearance::ear::Ears;
use rpg_tools_core::model::character::appearance::eye::{Eye, EyeShape, Eyes, PupilShape};
use rpg_tools_core::model::character::appearance::hair::Hair;
use rpg_tools_core::model::character::appearance::head::RealisticHeadShape::*;
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape};
use rpg_tools_core::model::character::appearance::mouth::Mouth;
use rpg_tools_core::model::character::appearance::skin::{Skin, SkinColor};
use rpg_tools_core::model::character::appearance::{Appearance, Size};
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;

pub mod utils;

fn main() {
    let eye_shapes = vec![EyeShape::Almond, EyeShape::Ellipse, EyeShape::Circle];
    let pupil_shapes = vec![
        PupilShape::Circle,
        PupilShape::HorizontalSlit,
        PupilShape::VerticalSlit,
    ];

    render_2_sets(
        "pupil_shapes.svg",
        pupil_shapes,
        eye_shapes,
        create_appearance,
    );
}

fn create_appearance(height: Length, pupil_shape: &PupilShape, eye_shape: &EyeShape) -> Appearance {
    Appearance::head(
        Head {
            ears: Ears::None,
            eyes: Eyes::Two {
                eye: Eye::Normal {
                    eye_shape: *eye_shape,
                    pupil_shape: *pupil_shape,
                    pupil_color: Color::Black,
                    background_color: Color::Yellow,
                },
                distance: Size::Medium,
            },
            hair: Hair::None,
            mouth: Mouth::None,
            shape: HeadShape::Realistic(Oval),
            skin: Skin::Skin(SkinColor::Light),
        },
        height,
    )
}
