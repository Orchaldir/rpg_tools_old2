extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::render_2_sets;
use rpg_tools_core::model::character::appearance::ear::{EarShape, Ears};
use rpg_tools_core::model::character::appearance::eye::{Eye, EyeShape, Eyes, PupilShape};
use rpg_tools_core::model::character::appearance::hair::{Hair, HairColor, Hairline, ShortHair};
use rpg_tools_core::model::character::appearance::head::RealisticHeadShape::*;
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape, RealisticHeadShape};
use rpg_tools_core::model::character::appearance::mouth::{Mouth, SpecialTeeth, TeethColor};
use rpg_tools_core::model::character::appearance::skin::{Skin, SkinColor};
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;
use rpg_tools_core::model::side::Side::Left;
use rpg_tools_core::model::size::Size;

pub mod utils;

fn main() {
    let shape_options = vec![
        EarShape::Pointed(Size::Low),
        EarShape::Pointed(Size::Medium),
        EarShape::Pointed(Size::High),
        EarShape::Round,
        EarShape::Square,
    ];
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
            hair: Hair::Short {
                style: ShortHair::SidePart(Left),
                hairline: Hairline::Round(Size::Medium),
                color: HairColor::Blond,
            },
            mouth: Mouth::Normal {
                width: Size::Medium,
                color: None,
                teeth: SpecialTeeth::None,
                teeth_color: TeethColor::White,
            },
            shape: HeadShape::Realistic(*face),
            skin: Skin::Skin(SkinColor::Light),
        },
        height,
    )
}
