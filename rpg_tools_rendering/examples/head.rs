extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::config::{create_border_options, create_head_config};
use rpg_tools_core::model::character::appearance::eye::{Eye, EyeShape, Eyes, PupilShape};
use rpg_tools_core::model::character::appearance::head::RealisticHeadShape::*;
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape};
use rpg_tools_core::model::character::appearance::skin::Skin;
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;
use rpg_tools_rendering::math::aabb2d::AABB;
use rpg_tools_rendering::renderer::color::WebColor;
use rpg_tools_rendering::renderer::svg::SvgBuilder;
use rpg_tools_rendering::renderer::Renderer;
use rpg_tools_rendering::rendering::character::{calculate_character_size, render_character};
use rpg_tools_rendering::rendering::config::RenderConfig;

pub mod utils;

fn main() {
    let config = RenderConfig {
        border: 500,
        line_color: WebColor::from_color(Color::Black),
        line_width: 50,
        cut_corners_u: 0.25,
        cut_corners_v: 0.25,
        cut_corners_n: 3,
        head: create_head_config(),
    };
    let options = create_border_options();

    for (i, realistic) in [Oval, Rectangle, Round, Square, TriangleDown, TriangleUp]
        .iter()
        .enumerate()
    {
        let appearance = Appearance::head(
            Head {
                eyes: Eyes::One(Eye::Normal {
                    eye_shape: EyeShape::Circle,
                    pupil_shape: PupilShape::VerticalSlit,
                    pupil_color: Color::Black,
                    background_color: Color::White,
                }),
                shape: HeadShape::Realistic(*realistic),
                skin: Skin::Scales(Color::Red),
            },
            Length::from_metre(1.0),
        );
        let size = calculate_character_size(&config, &appearance);
        let aabb = AABB::with_size(size);
        let mut svg_builder = SvgBuilder::new(size);

        svg_builder.render_rectangle(&aabb, &options);
        render_character(&mut svg_builder, &config, &aabb, &appearance);
        let svg = svg_builder.finish();
        svg.save(&format!("{}-{:?}.svg", i, realistic)).unwrap();
    }
}
