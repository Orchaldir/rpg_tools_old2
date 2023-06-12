extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::config::{create_border_options, create_head_config};
use rpg_tools_core::model::character::appearance::eye::{Eye, EyeDistance, EyeShape, Eyes};
use rpg_tools_core::model::character::appearance::head::RealisticHeadShape::*;
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape};
use rpg_tools_core::model::character::appearance::skin::Skin;
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;
use rpg_tools_rendering::math::aabb2d::AABB;
use rpg_tools_rendering::math::point2d::Point2d;
use rpg_tools_rendering::math::size2d::Size2d;
use rpg_tools_rendering::renderer::color::WebColor;
use rpg_tools_rendering::renderer::svg::SvgBuilder;
use rpg_tools_rendering::renderer::Renderer;
use rpg_tools_rendering::rendering::character::{
    calculate_character_size, calculate_size, render_character,
};
use rpg_tools_rendering::rendering::config::RenderConfig;

pub mod config;

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
    let height = Length::from_metre(1.0);
    let size = calculate_size(&config, height);
    let eye = Eye::Simple {
        eye_shape: EyeShape::Round,
        color: Color::Yellow,
    };
    let eyes_options = vec![
        Eyes::One(eye),
        Eyes::Two {
            eye,
            distance: EyeDistance::Medium,
        },
    ];
    let faces = vec![Oval, Rectangle, Round, Square, TriangleDown, TriangleUp];
    let svg_size = Size2d::new(
        faces.len() as u32 * size.width(),
        eyes_options.len() as u32 * size.height(),
    );
    let mut svg_builder = SvgBuilder::new(svg_size);
    let mut start = Point2d::default();

    for eyes in eyes_options {
        start.x = 0;

        for realistic in faces.iter() {
            let appearance = Appearance::head(
                Head {
                    eyes,
                    shape: HeadShape::Realistic(*realistic),
                    skin: Skin::Scales(Color::Red),
                },
                height,
            );
            let size = calculate_character_size(&config, &appearance);
            let aabb = AABB::new(start, size);

            svg_builder.render_rectangle(&aabb, &options);
            render_character(&mut svg_builder, &config, &aabb, &appearance);

            start.x += size.width() as i32;
        }

        start.y += size.height() as i32;
    }

    let svg = svg_builder.finish();
    svg.save("eyes.svg").unwrap();
}
