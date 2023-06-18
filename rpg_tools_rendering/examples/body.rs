extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::render_2_sets;
use rpg_tools_core::model::character::appearance::body::BodyShape::*;
use rpg_tools_core::model::character::appearance::body::BodyWidth::*;
use rpg_tools_core::model::character::appearance::body::{Body, BodyShape, BodyWidth};
use rpg_tools_core::model::character::appearance::eye::Eyes;
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape, RealisticHeadShape};
use rpg_tools_core::model::character::appearance::mouth::Mouth;
use rpg_tools_core::model::character::appearance::skin::{Skin, SkinColor};
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::length::Length;

pub mod utils;

fn main() {
    let rows = vec![Thin, Average, Wide];
    let columns = vec![Fat, Hourglass, Muscular, Rectangle];

    render_2_sets("bodies.svg", rows, columns, create_appearance);
}

fn create_appearance(height: Length, width: &BodyWidth, shape: &BodyShape) -> Appearance {
    let skin = Skin::Skin(SkinColor::Light);
    Appearance::humanoid(
        Body {
            shape: *shape,
            width: *width,
            skin,
        },
        Head {
            eyes: Eyes::None,
            mouth: Mouth::None,
            shape: HeadShape::Realistic(RealisticHeadShape::Oval),
            skin,
        },
        height,
    )
}
