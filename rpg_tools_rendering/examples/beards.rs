extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::render_2_sets;
use rpg_tools_core::model::character::appearance::beard::moustache::MoustacheStyle;
use rpg_tools_core::model::character::appearance::beard::moustache::MoustacheStyle::*;
use rpg_tools_core::model::character::appearance::beard::Beard;
use rpg_tools_core::model::character::appearance::ear::shape::EarShape;
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
use rpg_tools_core::model::size::Size::Medium;

pub mod utils;

fn main() {
    let short_options = vec![
        Beard::Stubble {
            color: Color::SaddleBrown,
        },
        create_moustache(Handlebar),
        create_moustache(Horseshoe),
        create_moustache(Pencil),
        create_moustache(Pyramid),
        create_moustache(Toothbrush),
        create_moustache(Walrus),
    ];

    render_2_sets(
        "beards.svg",
        short_options,
        HeadShape::get_all(),
        create_appearance,
    );
}

fn create_moustache(style: MoustacheStyle) -> Beard {
    Beard::Moustache {
        style,
        color: Color::SaddleBrown,
    }
}

fn create_appearance(height: Length, beard: &Beard, face: &HeadShape) -> Appearance {
    Appearance::head(
        Head {
            ears: Ears::Normal {
                shape: EarShape::Round,
                size: Medium,
            },
            eyes: Eyes::Two {
                eye: Eye::Normal {
                    eye_shape: EyeShape::Ellipse,
                    pupil_shape: PupilShape::Circle,
                    pupil_color: Color::Green,
                    background_color: Color::White,
                },
                distance: Medium,
            },
            hair: Hair::None,
            mouth: Mouth::Normal {
                beard: *beard,
                width: Medium,
                teeth: SpecialTeeth::None,
                teeth_color: TeethColor::White,
            },
            shape: *face,
            skin: Skin::Skin(SkinColor::Light),
        },
        height,
    )
}
