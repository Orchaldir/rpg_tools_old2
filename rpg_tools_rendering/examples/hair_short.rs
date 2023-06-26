extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::render_2_sets;
use rpg_tools_core::model::character::appearance::eye::Eyes;
use rpg_tools_core::model::character::appearance::hair::{Hair, HairColor, ShortHair};
use rpg_tools_core::model::character::appearance::head::RealisticHeadShape::*;
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape, RealisticHeadShape};
use rpg_tools_core::model::character::appearance::mouth::Mouth;
use rpg_tools_core::model::character::appearance::skin::{Skin, SkinColor};
use rpg_tools_core::model::character::appearance::{Appearance, Side};
use rpg_tools_core::model::length::Length;

pub mod utils;

fn main() {
    let eyes_options = vec![
        ShortHair::BuzzCut,
        ShortHair::CrewCut,
        ShortHair::SidePart(Side::Left),
        ShortHair::SidePart(Side::Right),
    ];
    let faces = vec![Oval, Rectangle, Round, Square, TriangleDown, TriangleUp];

    render_2_sets("hair_short.svg", eyes_options, faces, create_appearance);
}

fn create_appearance(height: Length, hair: &ShortHair, face: &RealisticHeadShape) -> Appearance {
    Appearance::head(
        Head {
            eyes: Eyes::None,
            hair: Hair::Short {
                style: *hair,
                color: HairColor::Brown,
            },
            mouth: Mouth::None,
            shape: HeadShape::Realistic(*face),
            skin: Skin::Skin(SkinColor::Light),
        },
        height,
    )
}
