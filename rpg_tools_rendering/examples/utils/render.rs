extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use crate::utils::config::{create_border_options, create_config, create_head_config};
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

pub fn render_2_sets<T, S>(
    filename: &str,
    rows: Vec<T>,
    columns: Vec<S>,
    create: fn(Length, &T, &S) -> Appearance,
) {
    let config = create_config();
    let options = create_border_options();
    let height = Length::from_metre(1.0);
    let size = calculate_size(&config, height);
    let svg_size = Size2d::new(
        columns.len() as u32 * size.width(),
        rows.len() as u32 * size.height(),
    );
    let mut svg_builder = SvgBuilder::new(svg_size);
    let mut start = Point2d::default();

    for eyes in rows.iter() {
        start.x = 0;

        for realistic in columns.iter() {
            let appearance = create(height, eyes, realistic);
            let size = calculate_character_size(&config, &appearance);
            let aabb = AABB::new(start, size);

            svg_builder.render_rectangle(&aabb, &options);
            render_character(&mut svg_builder, &config, &aabb, &appearance);

            start.x += size.width() as i32;
        }

        start.y += size.height() as i32;
    }

    let svg = svg_builder.finish();
    svg.save(filename).unwrap();
}
