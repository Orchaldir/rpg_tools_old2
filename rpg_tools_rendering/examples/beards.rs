extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::{add_names, render_2_sets};
use rpg_tools_core::model::character::appearance::beard::goatee::GoateeStyle;
use rpg_tools_core::model::character::appearance::beard::goatee::GoateeStyle::*;
use rpg_tools_core::model::character::appearance::beard::moustache::MoustacheStyle;
use rpg_tools_core::model::character::appearance::beard::moustache::MoustacheStyle::*;
use rpg_tools_core::model::character::appearance::beard::Beard;
use rpg_tools_core::model::character::appearance::hair::hairline::{Hairline, HairlineStyle};
use rpg_tools_core::model::character::appearance::hair::Hair;
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape};
use rpg_tools_core::model::character::appearance::mouth::Mouth;
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;
use rpg_tools_core::model::size::Size::Medium;

pub mod utils;

fn main() {
    let short_options = vec![
        (
            "Stubble".to_string(),
            Beard::Stubble {
                color: Color::SaddleBrown,
            },
        ),
        create_moustache(Handlebar),
        create_moustache(FuManchu),
        create_moustache(Pencil),
        create_moustache(Pyramid),
        create_moustache(Toothbrush),
        create_moustache(Walrus),
        create_goatee(GoatPatch),
        create_goatee(Goatee),
        create_goatee(SoulPatch),
        create_goatee(VanDyke),
        (
            "Handlebar - VanDyke".to_string(),
            Beard::GoateeAndMoustache {
                moustache: Handlebar,
                goatee: VanDyke,
                color: Color::SaddleBrown,
            },
        ),
    ];

    render_2_sets(
        "beards.svg",
        short_options,
        add_names(HeadShape::get_all()),
        create_appearance,
        false,
    );
}

fn create_goatee(goatee: GoateeStyle) -> (String, Beard) {
    (
        goatee.to_string(),
        Beard::Goatee {
            goatee,
            color: Color::SaddleBrown,
        },
    )
}

fn create_moustache(moustache: MoustacheStyle) -> (String, Beard) {
    (
        format!("{} Moustache", moustache),
        Beard::Moustache {
            moustache,
            color: Color::SaddleBrown,
        },
    )
}

fn create_appearance(height: Length, beard: &Beard, face: &HeadShape) -> Appearance {
    Appearance::head(
        Head {
            ears: Default::default(),
            eyes: Default::default(),
            hair: Hair::Short {
                style: Default::default(),
                hairline: Hairline::new(HairlineStyle::Round, Medium),
                color: Color::SaddleBrown,
            },
            mouth: Mouth::Simple {
                beard: *beard,
                width: Medium,
                teeth: Default::default(),
                teeth_color: Default::default(),
            },
            shape: *face,
            skin: Default::default(),
        },
        height,
    )
}
