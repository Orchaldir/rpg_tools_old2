extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::{add_names, render_2_sets};
use rpg_tools_core::model::appearance::color::Color;
use rpg_tools_core::model::appearance::length::Length;
use rpg_tools_core::model::appearance::size::Size;
use rpg_tools_core::model::character::appearance::head::Head;
use rpg_tools_core::model::character::appearance::mouth::{Mouth, SpecialTeeth, TeethColor};
use rpg_tools_core::model::character::appearance::skin::{Skin, SkinColor};
use rpg_tools_core::model::character::appearance::Appearance;
use Size::Large;

pub mod utils;

fn main() {
    let skin = vec![
        create_normal(SkinColor::Fair),
        create_normal(SkinColor::Light),
        create_normal(SkinColor::Medium),
        create_normal(SkinColor::Tan),
        create_normal(SkinColor::Dark),
        create_normal(SkinColor::VeryDark),
        create_exotic(Color::Green),
    ];

    render_2_sets(
        "teeth_colors.svg",
        skin,
        add_names(TeethColor::get_all()),
        create_appearance,
        false,
    );
}

fn create_normal(color: SkinColor) -> (String, Skin) {
    (format!("{}", color), Skin::normal(color))
}

fn create_exotic(color: Color) -> (String, Skin) {
    (format!("Exotic {}", color), Skin::exotic(color))
}

fn create_appearance(height: Length, skin: &Skin, color: &TeethColor) -> Appearance {
    Appearance::head(
        Head {
            ears: Default::default(),
            eyes: Default::default(),
            hair: Default::default(),
            mouth: Mouth::Simple {
                beard: Default::default(),
                width: Large,
                teeth: SpecialTeeth::LowerFangs { size: Large },
                teeth_color: *color,
            },
            shape: Default::default(),
            skin: *skin,
        },
        height,
    )
}
