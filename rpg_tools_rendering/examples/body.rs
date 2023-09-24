extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::{add_names, render_2_sets};
use rpg_tools_core::model::character::appearance::body::{Body, BodyShape};
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::length::Length;
use rpg_tools_core::model::width::Width;

pub mod utils;

fn main() {
    let rows = add_names(Width::get_all());
    let columns = add_names(BodyShape::get_all());

    render_2_sets("bodies.svg", rows, columns, create_appearance, false);
}

fn create_appearance(height: Length, width: &Width, shape: &BodyShape) -> Appearance {
    Appearance::humanoid(
        Body {
            shape: *shape,
            width: *width,
            skin: Default::default(),
            clothing: Default::default(),
        },
        Default::default(),
        height,
    )
}
