extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::render_2_sets;
use rpg_tools_core::model::character::appearance::beard::full::FullBeardStyle;
use rpg_tools_core::model::character::appearance::beard::Beard;
use rpg_tools_core::model::character::appearance::body::{Body, BodyShape};
use rpg_tools_core::model::character::appearance::ear::shape::EarShape;
use rpg_tools_core::model::character::appearance::ear::Ears;
use rpg_tools_core::model::character::appearance::eye::pupil::PupilShape;
use rpg_tools_core::model::character::appearance::eye::shape::EyeShape;
use rpg_tools_core::model::character::appearance::eye::{Eye, Eyes};
use rpg_tools_core::model::character::appearance::hair::hairline::Hairline;
use rpg_tools_core::model::character::appearance::hair::{Hair, ShortHair};
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape};
use rpg_tools_core::model::character::appearance::mouth::{Mouth, SpecialTeeth, TeethColor};
use rpg_tools_core::model::character::appearance::skin::{Skin, SkinColor};
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;
use rpg_tools_core::model::size::Size::Medium;
use rpg_tools_core::model::width::Width;

pub mod utils;

fn main() {
    let mut options = Vec::new();

    for style in FullBeardStyle::get_all() {
        for l in 1..6 {
            options.push(create_full(style, Length::from_metre(l as f32 * 0.2)));
        }
    }

    render_2_sets(
        "beards_full.svg",
        options,
        HeadShape::get_all(),
        create_appearance,
    );
}

fn create_full(style: FullBeardStyle, length: Length) -> Beard {
    Beard::FullBeard {
        style,
        length,
        color: Color::SaddleBrown,
    }
}

fn create_appearance(height: Length, beard: &Beard, face: &HeadShape) -> Appearance {
    Appearance::humanoid(
        Body {
            shape: BodyShape::Rectangle,
            width: Width::Average,
            skin: Skin::Skin(SkinColor::Light),
        },
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
                eyebrows: Default::default(),
                distance: Medium,
            },
            hair: Hair::Short {
                style: ShortHair::MiddlePart,
                hairline: Hairline::Round(Medium),
                color: Color::SaddleBrown,
            },
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
