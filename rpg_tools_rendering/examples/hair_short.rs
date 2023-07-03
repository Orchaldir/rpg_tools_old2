extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::render_2_sets;
use rpg_tools_core::model::character::appearance::eye::{Eye, EyeShape, Eyes, PupilShape};
use rpg_tools_core::model::character::appearance::hair::ShortHair::FlatTop;
use rpg_tools_core::model::character::appearance::hair::{Hair, HairColor, Hairline, ShortHair};
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape, RealisticHeadShape};
use rpg_tools_core::model::character::appearance::mouth::{Mouth, SpecialTeeth, TeethColor};
use rpg_tools_core::model::character::appearance::skin::{Skin, SkinColor};
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::character::appearance::Size::{High, Low, Medium};
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;
use rpg_tools_core::model::side::Side;
use Hairline::{Round, Straight, Triangle, WidowsPeak};
use ShortHair::{BuzzCut, MiddlePart, SidePart};
use Side::{Left, Right};

pub mod utils;

fn main() {
    let mut short_options = vec![
        create_hair(FlatTop(Low), Round(Medium)),
        create_hair(FlatTop(Medium), Straight(Medium)),
        create_hair(FlatTop(High), WidowsPeak(Medium)),
        create_hair(MiddlePart, Round(Low)),
        create_hair(MiddlePart, Round(Medium)),
        create_hair(MiddlePart, Round(High)),
        create_hair(SidePart(Left), Round(Low)),
        create_hair(SidePart(Right), Round(Low)),
    ];
    add_all_hairlines(&mut short_options, BuzzCut);

    render_2_sets(
        "hair_short.svg",
        short_options,
        RealisticHeadShape::get_all(),
        create_appearance,
    );
}

fn add_all_hairlines(short_options: &mut Vec<Hair>, style: ShortHair) {
    short_options.append(&mut vec![
        create_hair(style, Round(Low)),
        create_hair(style, Round(Medium)),
        create_hair(style, Round(High)),
        create_hair(style, Straight(Low)),
        create_hair(style, Straight(Medium)),
        create_hair(style, Straight(High)),
        create_hair(style, Triangle(Low)),
        create_hair(style, Triangle(Medium)),
        create_hair(style, Triangle(High)),
        create_hair(style, WidowsPeak(Low)),
        create_hair(style, WidowsPeak(Medium)),
        create_hair(style, WidowsPeak(High)),
    ]);
}

fn create_hair(style: ShortHair, hairline: Hairline) -> Hair {
    Hair::Short {
        style,
        hairline,
        color: HairColor::Brown,
    }
}

fn create_appearance(height: Length, hair: &Hair, face: &RealisticHeadShape) -> Appearance {
    Appearance::head(
        Head {
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
            shape: HeadShape::Realistic(*face),
            skin: Skin::Skin(SkinColor::Light),
        },
        height,
    )
}
