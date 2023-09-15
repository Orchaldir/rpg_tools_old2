extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::render_2_sets;
use rpg_tools_core::model::character::appearance::beard::Beard;
use rpg_tools_core::model::character::appearance::ear::shape::EarShape;
use rpg_tools_core::model::character::appearance::ear::Ears;
use rpg_tools_core::model::character::appearance::eye::brow::shape::EyebrowShape;
use rpg_tools_core::model::character::appearance::eye::brow::style::EyebrowStyle;
use rpg_tools_core::model::character::appearance::eye::brow::EyeBrows;
use rpg_tools_core::model::character::appearance::eye::pupil::PupilShape;
use rpg_tools_core::model::character::appearance::eye::shape::EyeShape;
use rpg_tools_core::model::character::appearance::eye::{Eye, Eyes};
use rpg_tools_core::model::character::appearance::hair::hairline::{Hairline, HairlineStyle};
use rpg_tools_core::model::character::appearance::hair::short::ShortHair;
use rpg_tools_core::model::character::appearance::hair::Hair;
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape};
use rpg_tools_core::model::character::appearance::mouth::{Mouth, SpecialTeeth, TeethColor};
use rpg_tools_core::model::character::appearance::skin::{Skin, SkinColor};
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;
use rpg_tools_core::model::size::Size::Medium;
use rpg_tools_core::model::width::Width;
use std::vec;

pub mod utils;

fn main() {
    let mut options = Vec::new();

    for is_unibrow in &[false, true] {
        for style in EyebrowStyle::get_all() {
            for width in Width::get_all() {
                options.push((*is_unibrow, style, width));
            }
        }
    }

    render_2_sets(
        "eyebrows.svg",
        options,
        EyebrowShape::get_all(),
        create_appearance,
        false,
    );
}

fn create_appearance(
    height: Length,
    options: &(bool, EyebrowStyle, Width),
    shape: &EyebrowShape,
) -> Appearance {
    let (is_unibrow, style, width) = *options;

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
                eyebrows: if is_unibrow {
                    EyeBrows::Unibrow {
                        color: Color::SaddleBrown,
                        shape: *shape,
                        style,
                        width,
                    }
                } else {
                    EyeBrows::Normal {
                        color: Color::SaddleBrown,
                        shape: *shape,
                        style,
                        width,
                    }
                },
                distance: Medium,
            },
            hair: Hair::Short {
                style: ShortHair::MiddlePart,
                hairline: Hairline::new(HairlineStyle::Round, Medium),
                color: Color::SaddleBrown,
            },
            mouth: Mouth::Simple {
                beard: Beard::None,
                width: Medium,
                teeth: SpecialTeeth::None,
                teeth_color: TeethColor::White,
            },
            shape: HeadShape::Round,
            skin: Skin::normal(SkinColor::Light),
        },
        height,
    )
}
