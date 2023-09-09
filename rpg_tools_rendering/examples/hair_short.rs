extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::appearance::create_head_with_hair;
use crate::utils::render::render_2_sets;
use rpg_tools_core::model::character::appearance::hair::hairline::Hairline;
use rpg_tools_core::model::character::appearance::hair::short::ShortHair;
use rpg_tools_core::model::character::appearance::hair::Hair;
use rpg_tools_core::model::character::appearance::head::HeadShape;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::side::Side;
use rpg_tools_core::model::size::Size::{Large, Medium, Small};
use Hairline::{Round, Straight, Triangle, WidowsPeak};
use ShortHair::{BuzzCut, FlatTop, MiddlePart, SidePart};
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
        create_head_with_hair,
        false,
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
        color: Color::SaddleBrown,
    }
}
