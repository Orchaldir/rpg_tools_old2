extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::appearance::create_humanoid_with_hair;
use crate::utils::render::{add_names, render_2_sets};
use rpg_tools_core::model::character::appearance::hair::ponytail::position::PonytailPosition;
use rpg_tools_core::model::character::appearance::hair::ponytail::style::PonytailStyle;
use rpg_tools_core::model::character::appearance::hair::ponytail::Ponytail;
use rpg_tools_core::model::character::appearance::hair::Hair;
use rpg_tools_core::model::character::appearance::head::HeadShape;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;

pub mod utils;

fn main() {
    let mut options = Vec::new();

    for position in PonytailPosition::get_all() {
        for style in PonytailStyle::get_all() {
            options.push(create_ponytail(position, style));
        }
    }

    render_2_sets(
        "hair_ponytail.svg",
        options,
        add_names(HeadShape::get_all()),
        create_humanoid_with_hair,
        true,
    );
}

fn create_ponytail(position: PonytailPosition, style: PonytailStyle) -> (String, Hair) {
    (
        format!("{} + {}", style, position),
        Hair::Ponytail(Ponytail {
            position,
            style,
            length: Length::from_metre(1.0),
            hairline: Default::default(),
            color: Color::SaddleBrown,
        }),
    )
}
