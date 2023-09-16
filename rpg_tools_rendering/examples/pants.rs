extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::render_2_sets;
use rpg_tools_core::model::character::appearance::body::{Body, BodyShape};
use rpg_tools_core::model::character::appearance::head::Head;
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::equipment::appearance::pants::{Pants, PantsStyle, Rise};
use rpg_tools_core::model::equipment::appearance::Clothing;
use rpg_tools_core::model::length::Length;
use rpg_tools_core::model::width::Width;

pub mod utils;

fn main() {
    let mut pants = vec![];

    for style in PantsStyle::get_all() {
        pants.push(create(style, Rise::Regular))
    }

    render_2_sets(
        "pants.svg",
        pants,
        BodyShape::get_all(),
        create_appearance,
        false,
    );
}

fn create(style: PantsStyle, rise: Rise) -> Pants {
    Pants {
        style,
        rise,
        color: Color::Blue,
    }
}

fn create_appearance(height: Length, pants: &Pants, shape: &BodyShape) -> Appearance {
    Appearance::humanoid(
        Body {
            shape: *shape,
            width: Width::default(),
            skin: Default::default(),
            clothing: Clothing::Simple { pants: *pants },
        },
        Head::default(),
        height,
    )
}
