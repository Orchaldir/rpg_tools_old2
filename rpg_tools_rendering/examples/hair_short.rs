extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::render::render_2_sets;
use rpg_tools_core::model::character::appearance::eye::{Eye, EyeShape, Eyes, PupilShape};
use rpg_tools_core::model::character::appearance::hair::{Hair, HairColor, Hairline, ShortHair};
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape, RealisticHeadShape};
use rpg_tools_core::model::character::appearance::mouth::{Mouth, SpecialTeeth, TeethColor};
use rpg_tools_core::model::character::appearance::skin::{Skin, SkinColor};
use rpg_tools_core::model::character::appearance::Size::{High, Low, Medium};
use rpg_tools_core::model::character::appearance::{Appearance, Side, Size};
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;
use Hairline::{Round, Straight, WidowsPeak};
use ShortHair::{BuzzCut, CrewCut, SidePart};
use Side::{Left, Right};

pub mod utils;

fn main() {
    let eyes_options = vec![
        create_hair(BuzzCut, Round(Low)),
        create_hair(BuzzCut, Round(Medium)),
        create_hair(BuzzCut, Round(High)),
        create_hair(BuzzCut, Straight(Low)),
        create_hair(BuzzCut, Straight(Medium)),
        create_hair(BuzzCut, Straight(High)),
        create_hair(BuzzCut, WidowsPeak(Low)),
        create_hair(BuzzCut, WidowsPeak(Medium)),
        create_hair(BuzzCut, WidowsPeak(High)),
        create_hair(CrewCut, Round(Low)),
        create_hair(CrewCut, Straight(Low)),
        create_hair(CrewCut, WidowsPeak(Low)),
        create_hair(SidePart(Left), Round(Low)),
        create_hair(SidePart(Right), Round(Low)),
    ];

    render_2_sets(
        "hair_short.svg",
        eyes_options,
        RealisticHeadShape::get_all(),
        create_appearance,
    );
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
