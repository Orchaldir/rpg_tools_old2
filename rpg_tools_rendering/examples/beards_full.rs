extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::{add_names, render_2_sets};
use rpg_tools_core::model::appearance::color::Color;
use rpg_tools_core::model::appearance::length::Length;
use rpg_tools_core::model::appearance::size::Size::Medium;
use rpg_tools_core::model::character::appearance::beard::full::FullBeardStyle;
use rpg_tools_core::model::character::appearance::beard::Beard;
use rpg_tools_core::model::character::appearance::hair::Hair;
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape};
use rpg_tools_core::model::character::appearance::mouth::Mouth;
use rpg_tools_core::model::character::appearance::Appearance;

pub mod utils;

fn main() {
    let mut options = Vec::new();

    for style in FullBeardStyle::get_all() {
        for l in 1..6 {
            options.push(create_full(style, Length::from_metre(l as f32 * 0.2)));
        }
    }

    render_2_sets(
        "beards_full.svg",
        options,
        add_names(HeadShape::get_all()),
        create_appearance,
        false,
    );
}

fn create_full(style: FullBeardStyle, length: Length) -> (String, Beard) {
    (
        format!("{} + {} m", style, length.to_metre()),
        Beard::FullBeard {
            style,
            length,
            color: Color::SaddleBrown,
        },
    )
}

fn create_appearance(height: Length, beard: &Beard, face: &HeadShape) -> Appearance {
    Appearance::humanoid(
        Default::default(),
        Head {
            ears: Default::default(),
            eyes: Default::default(),
            hair: Hair::Short {
                style: Default::default(),
                hairline: Default::default(),
                color: Color::SaddleBrown,
            },
            mouth: Mouth::Simple {
                beard: *beard,
                width: Medium,
                teeth: Default::default(),
                teeth_color: Default::default(),
            },
            shape: *face,
            skin: Default::default(),
        },
        height,
    )
}
