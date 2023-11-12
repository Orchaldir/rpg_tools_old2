extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::{add_names, render_2_sets};
use rpg_tools_core::model::appearance::color::Color;
use rpg_tools_core::model::appearance::length::Length;
use rpg_tools_core::model::character::appearance::body::{Body, BodyShape};
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::equipment::appearance::option::neckline::Neckline;
use rpg_tools_core::model::equipment::appearance::option::sleeve::SleeveStyle;
use rpg_tools_core::model::equipment::appearance::shirt::Shirt;
use rpg_tools_core::model::equipment::appearance::Clothing;

pub mod utils;

fn main() {
    let mut shirts = vec![];

    for neckline in Neckline::get_all() {
        for sleeve_style in SleeveStyle::get_all() {
            shirts.push(create(neckline, sleeve_style))
        }
    }

    render_2_sets(
        "shirts.svg",
        shirts,
        add_names(BodyShape::get_all()),
        create_appearance,
        false,
    );
}

fn create(neckline: Neckline, sleeve_style: SleeveStyle) -> (String, Shirt) {
    (
        format!("{} - {}", neckline, sleeve_style),
        Shirt {
            sleeve_style,
            neckline,
            color: Color::Aqua,
        },
    )
}

fn create_appearance(height: Length, shirt: &Shirt, shape: &BodyShape) -> Appearance {
    Appearance::humanoid(
        Body {
            shape: *shape,
            width: Default::default(),
            skin: Default::default(),
            clothing: Clothing::Simple {
                footwear: Default::default(),
                pants: Default::default(),
                shirt: *shirt,
                outerwear: Default::default(),
            },
        },
        Default::default(),
        height,
    )
}
