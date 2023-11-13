extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::{add_names, render_2_sets};
use rpg_tools_core::model::appearance::color::Color;
use rpg_tools_core::model::appearance::length::Length;
use rpg_tools_core::model::character::appearance::body::Body;
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::equipment::appearance::option::button::{Button, ButtonColumn};
use rpg_tools_core::model::equipment::appearance::option::neckline::Neckline;
use rpg_tools_core::model::equipment::appearance::outerwear::coat::{ClosingOption, Coat};
use rpg_tools_core::model::equipment::appearance::outerwear::Outerwear;
use rpg_tools_core::model::equipment::appearance::Clothing;

pub mod utils;

fn main() {
    let buttons = ButtonColumn {
        button: Button {
            size: Default::default(),
            color: Color::Silver,
        },
        count: 4,
    };
    let options = vec![
        ("None".to_string(), ClosingOption::None),
        (
            "Single Breasted".to_string(),
            ClosingOption::SingleBreasted { buttons },
        ),
        (
            "Double Breasted".to_string(),
            ClosingOption::DoubleBreasted { buttons },
        ),
        (
            "Zipper".to_string(),
            ClosingOption::Zipper { color: Color::Gray },
        ),
    ];

    render_2_sets(
        "coats-style.svg",
        add_names(Neckline::get_all()),
        options,
        create_appearance,
        false,
    );
}

fn create_appearance(height: Length, neckline: &Neckline, closing: &ClosingOption) -> Appearance {
    let coat = Coat {
        sleeve: Default::default(),
        length: Default::default(),
        neckline: *neckline,
        closing: *closing,
        color: Color::Blue,
        belt: None,
    };
    Appearance::humanoid(
        Body {
            shape: Default::default(),
            width: Default::default(),
            skin: Default::default(),
            clothing: Clothing::Simple {
                footwear: Default::default(),
                pants: Default::default(),
                shirt: Default::default(),
                outerwear: Outerwear::Coat(coat),
            },
        },
        Default::default(),
        height,
    )
}
