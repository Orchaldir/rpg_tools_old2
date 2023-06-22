extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::render_2_sets;
use rpg_tools_core::model::character::appearance::eye::Eyes;
use rpg_tools_core::model::character::appearance::head::RealisticHeadShape::*;
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape, RealisticHeadShape};
use rpg_tools_core::model::character::appearance::mouth::{Mouth, SpecialTeeth, TeethColor};
use rpg_tools_core::model::character::appearance::skin::Skin;
use rpg_tools_core::model::character::appearance::{Appearance, Size};
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;
use Size::High;

pub mod utils;

fn main() {
    let teeth_colors = vec![
        TeethColor::White,
        TeethColor::Yellow,
        TeethColor::Brown,
        TeethColor::Black,
    ];
    let faces = vec![Oval];

    render_2_sets("teeth_colors.svg", faces, teeth_colors, create_appearance);
}

fn create_appearance(height: Length, face: &RealisticHeadShape, color: &TeethColor) -> Appearance {
    Appearance::head(
        Head {
            eyes: Eyes::None,
            mouth: Mouth::Normal {
                width: High,
                color: None,
                teeth: SpecialTeeth::LowerFangs(High),
                teeth_color: *color,
            },
            shape: HeadShape::Realistic(*face),
            skin: Skin::Scales(Color::Green),
        },
        height,
    )
}
