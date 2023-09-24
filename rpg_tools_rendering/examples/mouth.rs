extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::{add_names, render_2_sets};
use rpg_tools_core::model::character::appearance::beard::Beard;
use rpg_tools_core::model::character::appearance::eye::{Eye, Eyes};
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape};
use rpg_tools_core::model::character::appearance::mouth::{Mouth, SpecialTeeth, TeethColor};
use rpg_tools_core::model::character::appearance::skin::Skin;
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;
use rpg_tools_core::model::size::Size;
use Size::{Large, Medium, Small};

pub mod utils;

fn main() {
    let shape_options = vec![
        create_circle(Small),
        create_circle(Medium),
        create_circle(Large),
        create_normal(Small, Small),
        create_normal(Small, Medium),
        create_normal(Small, Large),
        create_normal(Medium, Small),
        create_normal(Medium, Medium),
        create_normal(Medium, Large),
        create_normal(Large, Small),
        create_normal(Large, Medium),
        create_normal(Large, Large),
        create_female(Small),
        create_female(Medium),
        create_female(Large),
    ];
    let faces = add_names(HeadShape::get_all());

    render_2_sets("mouth.svg", shape_options, faces, create_appearance, false);
}

fn create_circle(size: Size) -> (String, Mouth) {
    (
        format!("Circle + {:?}", size),
        Mouth::Circle {
            size,
            teeth_color: TeethColor::White,
        },
    )
}

fn create_normal(width: Size, teeth: Size) -> (String, Mouth) {
    (
        format!("{:?} + {:?}", width, teeth),
        Mouth::Simple {
            beard: Beard::None,
            width,
            teeth: SpecialTeeth::LowerFangs { size: teeth },
            teeth_color: TeethColor::White,
        },
    )
}

fn create_female(width: Size) -> (String, Mouth) {
    (
        format!("Female + {:?}", width),
        Mouth::Female {
            width,
            color: Color::White,
            teeth: SpecialTeeth::None,
            teeth_color: TeethColor::White,
        },
    )
}

fn create_appearance(height: Length, mouth: &Mouth, face: &HeadShape) -> Appearance {
    Appearance::head(
        Head {
            ears: Default::default(),
            eyes: Eyes::Two {
                eye: Eye::Simple {
                    eye_shape: Default::default(),
                    color: Color::Yellow,
                },
                eyebrows: Default::default(),
                distance: Default::default(),
            },
            hair: Default::default(),
            mouth: *mouth,
            shape: *face,
            skin: Skin::exotic(Color::Red),
        },
        height,
    )
}
