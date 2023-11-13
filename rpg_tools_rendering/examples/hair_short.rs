extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::appearance::create_head_with_hair;
use crate::utils::render::{add_names, render_2_sets};
use rpg_tools_core::model::appearance::color::Color;
use rpg_tools_core::model::appearance::side::Side;
use rpg_tools_core::model::appearance::size::Size;
use rpg_tools_core::model::appearance::size::Size::{Large, Medium, Small};
use rpg_tools_core::model::character::appearance::hair::hairline::{Hairline, HairlineStyle};
use rpg_tools_core::model::character::appearance::hair::short::ShortHair;
use rpg_tools_core::model::character::appearance::hair::Hair;
use rpg_tools_core::model::character::appearance::head::HeadShape;
use HairlineStyle::{Round, Straight, Triangle, WidowsPeak};
use ShortHair::{BuzzCut, MiddlePart};
use Side::{Left, Right};

pub mod utils;

fn main() {
    let mut short_options = vec![
        create_hair(ShortHair::flat_top(Small), Round, Medium),
        create_hair(ShortHair::flat_top(Medium), Straight, Medium),
        create_hair(ShortHair::flat_top(Large), WidowsPeak, Medium),
        create_hair(MiddlePart, Round, Small),
        create_hair(MiddlePart, Round, Medium),
        create_hair(MiddlePart, Round, Large),
        create_hair(ShortHair::side_part(Left), Round, Small),
        create_hair(ShortHair::side_part(Right), Round, Small),
    ];
    add_all_hairlines(&mut short_options, BuzzCut);

    render_2_sets(
        "hair_short.svg",
        short_options,
        add_names(HeadShape::get_all()),
        create_head_with_hair,
        false,
    );
}

fn add_all_hairlines(short_options: &mut Vec<(String, Hair)>, style: ShortHair) {
    short_options.append(&mut vec![
        create_hair(style, Round, Small),
        create_hair(style, Round, Medium),
        create_hair(style, Round, Large),
        create_hair(style, Straight, Small),
        create_hair(style, Straight, Medium),
        create_hair(style, Straight, Large),
        create_hair(style, Triangle, Small),
        create_hair(style, Triangle, Medium),
        create_hair(style, Triangle, Large),
        create_hair(style, WidowsPeak, Small),
        create_hair(style, WidowsPeak, Medium),
        create_hair(style, WidowsPeak, Large),
    ]);
}

fn create_hair(style: ShortHair, hairline: HairlineStyle, size: Size) -> (String, Hair) {
    (
        format!("{:?} + {:?}", style, hairline),
        Hair::Short {
            style,
            hairline: Hairline::new(hairline, size),
            color: Color::SaddleBrown,
        },
    )
}
