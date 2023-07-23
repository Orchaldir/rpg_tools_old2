extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::render_2_sets;
use rpg_tools_core::model::character::appearance::ear::{EarShape, Ears};
use rpg_tools_core::model::character::appearance::eye::{Eye, EyeShape, Eyes, PupilShape};
use rpg_tools_core::model::character::appearance::hair::ShortHair::FlatTop;
use rpg_tools_core::model::character::appearance::hair::{Hair, HairColor, Hairline, ShortHair};
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape};
use rpg_tools_core::model::character::appearance::mouth::{Mouth, SpecialTeeth, TeethColor};
use rpg_tools_core::model::character::appearance::skin::{Skin, SkinColor};
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;
use rpg_tools_core::model::side::Side;
use rpg_tools_core::model::size::Size::{Large, Medium, Small};
use Hairline::{Round, Straight, Triangle, WidowsPeak};
use ShortHair::{BuzzCut, MiddlePart, SidePart};
use Side::{Left, Right};

pub mod utils;

fn main() {
    let mut short_options = vec![
        create_hair(FlatTop(Small), Round(Medium)),
        create_hair(FlatTop(Medium), Straight(Medium)),
        create_hair(FlatTop(Large), WidowsPeak(Medium)),
        create_hair(MiddlePart, Round(Small)),
        create_hair(MiddlePart, Round(Medium)),
        create_hair(MiddlePart, Round(Large)),
        create_hair(SidePart(Left), Round(Small)),
        create_hair(SidePart(Right), Round(Small)),
    ];
    add_all_hairlines(&mut short_options, BuzzCut);

    render_2_sets(
        "hair_short.svg",
        short_options,
        HeadShape::get_all(),
        create_appearance,
    );
}

fn add_all_hairlines(short_options: &mut Vec<Hair>, style: ShortHair) {
    short_options.append(&mut vec![
        create_hair(style, Round(Small)),
        create_hair(style, Round(Medium)),
        create_hair(style, Round(Large)),
        create_hair(style, Straight(Small)),
        create_hair(style, Straight(Medium)),
        create_hair(style, Straight(Large)),
        create_hair(style, Triangle(Small)),
        create_hair(style, Triangle(Medium)),
        create_hair(style, Triangle(Large)),
        create_hair(style, WidowsPeak(Small)),
        create_hair(style, WidowsPeak(Medium)),
        create_hair(style, WidowsPeak(Large)),
    ]);
}

fn create_hair(style: ShortHair, hairline: Hairline) -> Hair {
    Hair::Short {
        style,
        hairline,
        color: HairColor::Brown,
    }
}

fn create_appearance(height: Length, hair: &Hair, face: &HeadShape) -> Appearance {
    Appearance::head(
        Head {
            ears: Ears::Normal {
                shape: EarShape::Round,
            },
            eyes: Eyes::Two {
                eye: Eye::Normal {
                    eye_shape: EyeShape::Ellipse,
                    pupil_shape: PupilShape::Circle,
                    pupil_color: Color::Green,
                    background_color: Color::White,
                },
                distance: Medium,
            },
            hair: *hair,
            mouth: Mouth::Normal {
                width: Medium,
                color: None,
                teeth: SpecialTeeth::None,
                teeth_color: TeethColor::White,
            },
            shape: *face,
            skin: Skin::Skin(SkinColor::Light),
        },
        height,
    )
}
