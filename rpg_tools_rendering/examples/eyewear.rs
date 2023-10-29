extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::{add_names, render_2_sets};
use rpg_tools_core::model::character::appearance::eye::{Eye, Eyes};
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape};
use rpg_tools_core::model::character::appearance::skin::{Skin, SkinColor};
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::equipment::appearance::eyewear::{
    Eyewear, FrameType, LensShape, LensStyle,
};
use rpg_tools_core::model::length::Length;

pub mod utils;

fn main() {
    let mut options = Vec::new();

    for frame_type in FrameType::get_all() {
        for lens_shape in LensShape::get_all() {
            options.push(create_eyewear(frame_type, lens_shape));
        }
    }

    render_2_sets(
        "eyewear-glasses.svg",
        options,
        add_names(HeadShape::get_all()),
        create_appearance,
        false,
    );
}

fn create_eyewear(frame_type: FrameType, lens_shape: LensShape) -> (String, Eyes) {
    let eye = Eye::Normal {
        eye_shape: Default::default(),
        pupil_shape: Default::default(),
        pupil_color: Color::Green,
        background_color: Color::White,
    };
    (
        format!("{} + {}", frame_type, lens_shape),
        Eyes::Two {
            eye,
            eyebrows: Default::default(),
            distance: Default::default(),
            eyewear: Eyewear::Glasses {
                style: LensStyle {
                    frame_color: Color::Black,
                    frame_type,
                    lens_color: Color::Aqua,
                    lens_shape,
                },
            },
        },
    )
}

fn create_appearance(height: Length, eyes: &Eyes, face: &HeadShape) -> Appearance {
    Appearance::head(
        Head {
            ears: Default::default(),
            eyes: *eyes,
            hair: Default::default(),
            mouth: Default::default(),
            shape: *face,
            skin: Skin::normal(SkinColor::Light),
        },
        height,
    )
}
