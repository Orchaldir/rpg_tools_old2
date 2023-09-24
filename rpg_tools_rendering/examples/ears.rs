extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::{add_names, render_2_sets};
use rpg_tools_core::model::character::appearance::ear::shape::EarShape;
use rpg_tools_core::model::character::appearance::ear::Ears;
use rpg_tools_core::model::character::appearance::hair::short::ShortHair;
use rpg_tools_core::model::character::appearance::hair::Hair;
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape};
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
    let faces = add_names(HeadShape::get_all());

    render_2_sets("ears.svg", shape_options, faces, create_appearance, false);
}

fn create(shape: EarShape, size: Size) -> (String, Ears) {
    (format!("{} {}", size, shape), Ears::Normal { shape, size })
}

fn create_appearance(height: Length, ears: &Ears, face: &HeadShape) -> Appearance {
    Appearance::head(
        Head {
            ears: *ears,
            eyes: Default::default(),
            hair: Hair::Short {
                style: ShortHair::SidePart { side: Left },
                hairline: Default::default(),
                color: Color::Yellow,
            },
            mouth: Default::default(),
            shape: *face,
            skin: Default::default(),
        },
        height,
    )
}
