extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::{add_names, render_2_sets};
use rpg_tools_core::model::character::appearance::body::Body;
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::equipment::appearance::option::neckline::Neckline;
use rpg_tools_core::model::equipment::appearance::outerwear::{ClosingOption, Coat, Outerwear};
use rpg_tools_core::model::equipment::appearance::Clothing;
use rpg_tools_core::model::length::Length;

pub mod utils;

fn main() {
    let options = vec![(
        "Zipper".to_string(),
        ClosingOption::Zipper { color: Color::Gray },
    )];

    render_2_sets(
        "coats-style.svg",
        add_names(Neckline::get_all()),
        options,
        create_appearance,
        false,
    );
}

fn create_appearance(height: Length, neckline: &Neckline, closing: &ClosingOption) -> Appearance {
    Appearance::humanoid(
        Body {
            shape: Default::default(),
            width: Default::default(),
            skin: Default::default(),
            clothing: Clothing::Simple {
                footwear: Default::default(),
                pants: Default::default(),
                shirt: Default::default(),
                outerwear: Outerwear::Coat {
                    style: Coat {
                        sleeve: Default::default(),
                        length: Default::default(),
                        neckline: *neckline,
                        closing: *closing,
                        color: Color::Blue,
                        belt: None,
                    },
                },
            },
        },
        Default::default(),
        height,
    )
}
