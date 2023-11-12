extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::{add_names, render_2_sets};
use rpg_tools_core::model::appearance::color::Color;
use rpg_tools_core::model::appearance::length::Length;
use rpg_tools_core::model::appearance::width::Width;
use rpg_tools_core::model::character::appearance::eye::brow::shape::EyebrowShape;
use rpg_tools_core::model::character::appearance::eye::brow::style::EyebrowStyle;
use rpg_tools_core::model::character::appearance::eye::brow::EyeBrows;
use rpg_tools_core::model::character::appearance::eye::Eyes;
use rpg_tools_core::model::character::appearance::hair::Hair;
use rpg_tools_core::model::character::appearance::head::Head;
use rpg_tools_core::model::character::appearance::Appearance;

pub mod utils;

fn main() {
    let mut options = Vec::new();

    for is_unibrow in &[false, true] {
        for style in EyebrowStyle::get_all() {
            for width in Width::get_all() {
                options.push((
                    format!("{} {} {}", is_unibrow, style, width),
                    (*is_unibrow, style, width),
                ));
            }
        }
    }

    render_2_sets(
        "eyebrows.svg",
        options,
        add_names(EyebrowShape::get_all()),
        create_appearance,
        false,
    );
}

fn create_appearance(
    height: Length,
    options: &(bool, EyebrowStyle, Width),
    shape: &EyebrowShape,
) -> Appearance {
    let (is_unibrow, style, width) = *options;

    Appearance::head(
        Head {
            ears: Default::default(),
            eyes: Eyes::Two {
                eye: Default::default(),
                eyebrows: if is_unibrow {
                    EyeBrows::Unibrow {
                        color: Color::SaddleBrown,
                        shape: *shape,
                        style,
                        width,
                    }
                } else {
                    EyeBrows::Normal {
                        color: Color::SaddleBrown,
                        shape: *shape,
                        style,
                        width,
                    }
                },
                distance: Default::default(),
                eyewear: Default::default(),
            },
            hair: Hair::Short {
                style: Default::default(),
                hairline: Default::default(),
                color: Color::SaddleBrown,
            },
            mouth: Default::default(),
            shape: Default::default(),
            skin: Default::default(),
        },
        height,
    )
}
