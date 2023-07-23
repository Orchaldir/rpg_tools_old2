extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::render_2_sets;
use rpg_tools_core::model::character::appearance::ear::Ears;
use rpg_tools_core::model::character::appearance::eye::{Eye, EyeShape, Eyes};
use rpg_tools_core::model::character::appearance::hair::Hair;
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape};
use rpg_tools_core::model::character::appearance::mouth::{Mouth, SpecialTeeth, TeethColor};
use rpg_tools_core::model::character::appearance::skin::Skin;
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;
use rpg_tools_core::model::size::Size;
use Size::{High, Low, Medium};

pub mod utils;

fn main() {
    let shape_options = vec![
        create_circle(Low),
        create_circle(Medium),
        create_circle(High),
        create_normal(Low, Low),
        create_normal(Low, Medium),
        create_normal(Low, High),
        create_normal(Medium, Low),
        create_normal(Medium, Medium),
        create_normal(Medium, High),
        create_normal(High, Low),
        create_normal(High, Medium),
        create_normal(High, High),
    ];
    let faces = HeadShape::get_all();

    render_2_sets("mouth.svg", shape_options, faces, create_appearance);
}

fn create_circle(size: Size) -> Mouth {
    Mouth::Circle {
        size,
        teeth_color: TeethColor::White,
    }
}

fn create_normal(width: Size, teeth: Size) -> Mouth {
    Mouth::Normal {
        width,
        color: None,
        teeth: SpecialTeeth::LowerFangs(teeth),
        teeth_color: TeethColor::White,
    }
}

fn create_appearance(height: Length, mouth: &Mouth, face: &HeadShape) -> Appearance {
    Appearance::head(
        Head {
            ears: Ears::None,
            eyes: Eyes::Two {
                eye: Eye::Simple {
                    eye_shape: EyeShape::Ellipse,
                    color: Color::Yellow,
                },
                distance: Medium,
            },
            hair: Hair::None,
            mouth: *mouth,
            shape: *face,
            skin: Skin::Scales(Color::Red),
        },
        height,
    )
}
