extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::{add_names, render_2_sets};
use rpg_tools_core::model::character::appearance::body::{Body, BodyShape};
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::equipment::appearance::option::button::{Button, ButtonColumn};
use rpg_tools_core::model::equipment::appearance::outerwear::coat::{
    ClosingOption, Coat, OuterwearLength,
};
use rpg_tools_core::model::equipment::appearance::outerwear::Outerwear;
use rpg_tools_core::model::equipment::appearance::Clothing;
use rpg_tools_core::model::length::Length;

pub mod utils;

fn main() {
    let mut lengths = vec![];

    for length in OuterwearLength::get_all() {
        lengths.push(create(length))
    }

    render_2_sets(
        "coats-length.svg",
        lengths,
        add_names(BodyShape::get_all()),
        create_appearance,
        false,
    );
}

fn create(length: OuterwearLength) -> (String, Coat) {
    (
        length.to_string(),
        Coat {
            sleeve: Default::default(),
            length,
            neckline: Default::default(),
            closing: ClosingOption::DoubleBreasted {
                buttons: ButtonColumn {
                    button: Button {
                        size: Default::default(),
                        color: Color::Orange,
                    },
                    count: 5,
                },
            },
            color: Color::Blue,
            belt: None,
        },
    )
}

fn create_appearance(height: Length, coat: &Coat, shape: &BodyShape) -> Appearance {
    Appearance::humanoid(
        Body {
            shape: *shape,
            width: Default::default(),
            skin: Default::default(),
            clothing: Clothing::Simple {
                footwear: Default::default(),
                pants: Default::default(),
                shirt: Default::default(),
                outerwear: Outerwear::Coat { style: *coat },
            },
        },
        Default::default(),
        height,
    )
}
