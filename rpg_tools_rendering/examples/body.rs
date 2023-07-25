extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::render_2_sets;
use rpg_tools_core::model::character::appearance::body::BodyShape::*;
use rpg_tools_core::model::character::appearance::body::{Body, BodyShape};
use rpg_tools_core::model::character::appearance::ear::Ears;
use rpg_tools_core::model::character::appearance::eye::Eyes;
use rpg_tools_core::model::character::appearance::hair::Hair;
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape};
use rpg_tools_core::model::character::appearance::mouth::Mouth;
use rpg_tools_core::model::character::appearance::skin::{Skin, SkinColor};
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::length::Length;
use rpg_tools_core::model::width::Width;
use rpg_tools_core::model::width::Width::*;

pub mod utils;

fn main() {
    let rows = vec![Thin, Average, Wide];
    let columns = vec![Fat, Hourglass, Muscular, Rectangle];

    render_2_sets("bodies.svg", rows, columns, create_appearance);
}

fn create_appearance(height: Length, width: &Width, shape: &BodyShape) -> Appearance {
    let skin = Skin::Skin(SkinColor::Light);
    Appearance::humanoid(
        Body {
            shape: *shape,
            width: *width,
            skin,
        },
        Head {
            ears: Ears::None,
            eyes: Eyes::None,
            hair: Hair::None,
            mouth: Mouth::None,
            shape: HeadShape::Oval,
            skin,
        },
        height,
    )
}
