extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::render_2_sets;
use rpg_tools_core::model::character::appearance::beard::Beard;
use rpg_tools_core::model::character::appearance::ear::Ears;
use rpg_tools_core::model::character::appearance::eye::pupil::PupilShape;
use rpg_tools_core::model::character::appearance::eye::shape::EyeShape;
use rpg_tools_core::model::character::appearance::eye::{Eye, Eyes};
use rpg_tools_core::model::character::appearance::hair::Hair;
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape};
use rpg_tools_core::model::character::appearance::mouth::{Mouth, SpecialTeeth, TeethColor};
use rpg_tools_core::model::character::appearance::skin::{Skin, SkinColor};
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;
use rpg_tools_core::model::size::Size;
use Size::Large;

pub mod utils;

fn main() {
    let teeth_colors = vec![TeethColor::White, TeethColor::Yellow, TeethColor::Brown];
    let skin = vec![
        Skin::skin(SkinColor::Fair),
        Skin::skin(SkinColor::Light),
        Skin::skin(SkinColor::Medium),
        Skin::skin(SkinColor::Tan),
        Skin::skin(SkinColor::Dark),
        Skin::skin(SkinColor::VeryDark),
        Skin::exotic(Color::Green),
    ];

    render_2_sets(
        "teeth_colors.svg",
        skin,
        teeth_colors,
        create_appearance,
        false,
    );
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
                eyebrows: Default::default(),
                distance: Size::Small,
            },
            hair: Hair::None,
            mouth: Mouth::Simple {
                beard: Beard::None,
                width: Large,
                teeth: SpecialTeeth::LowerFangs { size: Large },
                teeth_color: *color,
            },
            shape: HeadShape::Oval,
            skin: *skin,
        },
        height,
    )
}
