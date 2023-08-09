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
use rpg_tools_core::model::character::appearance::hair::hairline::Hairline;
use rpg_tools_core::model::character::appearance::hair::{Hair, ShortHair};
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape};
use rpg_tools_core::model::character::appearance::mouth::{Mouth, SpecialTeeth, TeethColor};
use rpg_tools_core::model::character::appearance::skin::{Skin, SkinColor};
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;
use rpg_tools_core::model::size::Size::Medium;

pub mod utils;

fn main() {
    let short_options = EyebrowStyle::get_all()
        .iter()
        .flat_map(|s| vec![(true, *s), (false, *s)])
        .collect();

    render_2_sets(
        "eyebrows.svg",
        short_options,
        EyebrowShape::get_all(),
        create_appearance,
    );
}

fn create_appearance(
    height: Length,
    options: &(bool, EyebrowStyle),
    shape: &EyebrowShape,
) -> Appearance {
    let (is_unibrow, style) = *options;

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
                    }
                } else {
                    EyeBrows::Normal {
                        color: Color::SaddleBrown,
                        shape: *shape,
                        style,
                    }
                },
                distance: Medium,
            },
            hair: Hair::Short {
                style: ShortHair::MiddlePart,
                hairline: Hairline::Round(Medium),
                color: Color::SaddleBrown,
            },
            mouth: Mouth::Normal {
                beard: Beard::None,
                width: Medium,
                teeth: SpecialTeeth::None,
                teeth_color: TeethColor::White,
            },
            shape: HeadShape::Round,
            skin: Skin::Skin(SkinColor::Light),
        },
        height,
    )
}
