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
    render_2_sets(
        "eyewear-glasses.svg",
        add_names(LensShape::get_all()),
        add_names(FrameType::get_all()),
        create_appearance,
        false,
    );
}

fn create_appearance(height: Length, lens_shape: &LensShape, frame_type: &FrameType) -> Appearance {
    let eye = Eye::Normal {
        eye_shape: Default::default(),
        pupil_shape: Default::default(),
        pupil_color: Color::Green,
        background_color: Color::White,
    };
    Appearance::head(
        Head {
            ears: Default::default(),
            eyes: Eyes::Two {
                eye,
                eyebrows: Default::default(),
                distance: Default::default(),
                eyewear: Eyewear::Glasses {
                    style: LensStyle {
                        frame_color: Color::Black,
                        frame_type: *frame_type,
                        lens_color: Color::Aqua,
                        lens_shape: *lens_shape,
                    },
                },
            },
            hair: Default::default(),
            mouth: Default::default(),
            shape: HeadShape::Oval,
            skin: Skin::normal(SkinColor::Light),
        },
        height,
    )
}
