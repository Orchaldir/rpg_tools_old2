extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::render_2_sets;
use rpg_tools_core::model::character::appearance::beard::Beard;
use rpg_tools_core::model::character::appearance::ear::shape::EarShape;
use rpg_tools_core::model::character::appearance::ear::Ears;
use rpg_tools_core::model::character::appearance::eye::pupil::PupilShape;
use rpg_tools_core::model::character::appearance::eye::shape::EyeShape;
use rpg_tools_core::model::character::appearance::eye::{Eye, Eyes};
use rpg_tools_core::model::character::appearance::hair::hairline::Hairline;
use rpg_tools_core::model::character::appearance::hair::short::ShortHair;
use rpg_tools_core::model::character::appearance::hair::Hair;
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape};
use rpg_tools_core::model::character::appearance::mouth::{Mouth, SpecialTeeth, TeethColor};
use rpg_tools_core::model::character::appearance::skin::{Skin, SkinColor};
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;
use rpg_tools_core::model::side::Side::Left;
use rpg_tools_core::model::size::Size;
use EarShape::*;
use Size::*;

pub mod utils;

fn main() {
    let shape_options = vec![
        create(Pointed, Small),
        create(Pointed, Medium),
        create(Pointed, Large),
        create(Round, Small),
        create(Round, Medium),
        create(Round, Large),
        create(Square, Medium),
    ];
    let faces = HeadShape::get_all();

    render_2_sets("ears.svg", shape_options, faces, create_appearance, false);
}

fn create(shape: EarShape, size: Size) -> Ears {
    Ears::Normal { shape, size }
}

fn create_appearance(height: Length, ears: &Ears, face: &HeadShape) -> Appearance {
    Appearance::head(
        Head {
            ears: *ears,
            eyes: Eyes::Two {
                eye: Eye::Normal {
                    eye_shape: EyeShape::Circle,
                    pupil_shape: PupilShape::Circle,
                    pupil_color: Color::Gray,
                    background_color: Color::White,
                },
                eyebrows: Default::default(),
                distance: Medium,
            },
            hair: Hair::Short {
                style: ShortHair::SidePart { side: Left },
                hairline: Hairline::Round { size: Medium },
                color: Color::Yellow,
            },
            mouth: Mouth::Simple {
                beard: Beard::None,
                width: Medium,
                teeth: SpecialTeeth::None,
                teeth_color: TeethColor::White,
            },
            shape: *face,
            skin: Skin::normal(SkinColor::Light),
        },
        height,
    )
}
