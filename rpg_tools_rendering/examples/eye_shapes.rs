extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::render_2_sets;
use rpg_tools_core::model::character::appearance::ear::Ears;
use rpg_tools_core::model::character::appearance::eye::brow::EyeBrows;
use rpg_tools_core::model::character::appearance::eye::shape::EyeShape;
use rpg_tools_core::model::character::appearance::eye::{Eye, Eyes};
use rpg_tools_core::model::character::appearance::hair::Hair;
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape};
use rpg_tools_core::model::character::appearance::mouth::Mouth;
use rpg_tools_core::model::character::appearance::skin::Skin;
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;
use rpg_tools_core::model::size::Size;

pub mod utils;

fn main() {
    let shape_options = vec![EyeShape::Almond, EyeShape::Ellipse, EyeShape::Circle];
    let faces = HeadShape::get_all();

    render_2_sets("eye_shapes.svg", shape_options, faces, create_appearance);
}

fn create_appearance(height: Length, shape: &EyeShape, face: &HeadShape) -> Appearance {
    Appearance::head(
        Head {
            ears: Ears::None,
            eyes: Eyes::Two {
                eye: Eye::Simple {
                    eye_shape: *shape,
                    color: Color::Yellow,
                },
                eyebrows: EyeBrows::None,
                distance: Size::Medium,
            },
            hair: Hair::None,
            mouth: Mouth::None,
            shape: *face,
            skin: Skin::Scales(Color::Red),
        },
        height,
    )
}
