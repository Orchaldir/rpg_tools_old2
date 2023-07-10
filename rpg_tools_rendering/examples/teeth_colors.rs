extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::render_2_sets;
use rpg_tools_core::model::character::appearance::ear::Ears;
use rpg_tools_core::model::character::appearance::eye::{Eye, EyeShape, Eyes, PupilShape};
use rpg_tools_core::model::character::appearance::hair::Hair;
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape, RealisticHeadShape};
use rpg_tools_core::model::character::appearance::mouth::{Mouth, SpecialTeeth, TeethColor};
use rpg_tools_core::model::character::appearance::skin::{Skin, SkinColor};
use rpg_tools_core::model::character::appearance::{Appearance, Size};
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;
use Size::High;

pub mod utils;

fn main() {
    let teeth_colors = vec![TeethColor::White, TeethColor::Yellow, TeethColor::Brown];
    let skin = vec![
        Skin::Skin(SkinColor::Fair),
        Skin::Skin(SkinColor::Light),
        Skin::Skin(SkinColor::Medium),
        Skin::Skin(SkinColor::Tan),
        Skin::Skin(SkinColor::Dark),
        Skin::Skin(SkinColor::VeryDark),
        Skin::Skin(SkinColor::Exotic(Color::Green)),
    ];

    render_2_sets("teeth_colors.svg", skin, teeth_colors, create_appearance);
}

fn create_appearance(height: Length, skin: &Skin, color: &TeethColor) -> Appearance {
    Appearance::head(
        Head {
            ears: Ears::None,
            eyes: Eyes::Two {
                eye: Eye::Normal {
                    eye_shape: EyeShape::Ellipse,
                    pupil_shape: PupilShape::Circle,
                    pupil_color: Color::Blue,
                    background_color: Color::White,
                },
                distance: Size::Low,
            },
            hair: Hair::None,
            mouth: Mouth::Normal {
                width: High,
                color: None,
                teeth: SpecialTeeth::LowerFangs(High),
                teeth_color: *color,
            },
            shape: HeadShape::Realistic(RealisticHeadShape::Oval),
            skin: *skin,
        },
        height,
    )
}
