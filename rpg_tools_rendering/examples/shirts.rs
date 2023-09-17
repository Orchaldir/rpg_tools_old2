extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::render_2_sets;
use rpg_tools_core::model::character::appearance::body::{Body, BodyShape};
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::equipment::appearance::shirt::{Neckline, Shirt, SleeveStyle};
use rpg_tools_core::model::equipment::appearance::Clothing;
use rpg_tools_core::model::length::Length;
use rpg_tools_core::model::width::Width;

pub mod utils;

fn main() {
    let mut shirts = vec![];

    for neckline in Neckline::get_all() {
        for sleeve_style in SleeveStyle::get_all() {
            shirts.push(Shirt {
                sleeve_style,
                neckline,
                color: Color::Aqua,
            })
        }
    }

    render_2_sets(
        "shirts.svg",
        shirts,
        BodyShape::get_all(),
        create_appearance,
        false,
    );
}

fn create_appearance(height: Length, shirt: &Shirt, shape: &BodyShape) -> Appearance {
    Appearance::humanoid(
        Body {
            shape: *shape,
            width: Width::default(),
            skin: Default::default(),
            clothing: Clothing::Simple {
                pants: Default::default(),
                shirt: *shirt,
            },
        },
        Default::default(),
        height,
    )
}
