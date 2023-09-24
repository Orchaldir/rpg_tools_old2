extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;
use rpg_tools_rendering::math::aabb2d::AABB;
use rpg_tools_rendering::math::orientation::Orientation;
use rpg_tools_rendering::math::point2d::Point2d;
use rpg_tools_rendering::math::size2d::Size2d;
use rpg_tools_rendering::renderer::color::WebColor;
use rpg_tools_rendering::renderer::svg::SvgBuilder;
use rpg_tools_rendering::renderer::text::TextOptions;
use rpg_tools_rendering::renderer::Renderer;
use rpg_tools_rendering::rendering::character::{
    calculate_character_size, calculate_size, render_character_from_back,
    render_character_from_front,
};
use rpg_tools_rendering::rendering::config::example::{create_border_options, create_config};
use std::fmt::Display;

pub fn render_2_sets<T, S>(
    filename: &str,
    rows: Vec<(String, T)>,
    columns: Vec<(String, S)>,
    create: fn(Length, &T, &S) -> Appearance,
    back_too: bool,
) {
    let config = create_config();
    let options = create_border_options();
    let height = Length::from_metre(1.8);
    let size = calculate_size(&config, height);
    let row_size = if back_too { 2 } else { 1 };
    let row_count = rows.len() as u32 * row_size;
    let svg_size = Size2d::new(
        columns.len() as u32 * size.width(),
        row_count * size.height(),
    );
    let mut svg_builder = SvgBuilder::new(svg_size);
    let mut start = Point2d::default();
    let text_size = size.height() / 15;
    let text_options = TextOptions::new(WebColor::from_color(Color::Black), text_size);
    let column_text_offset = Point2d::new((size.width() / 2) as i32, text_size as i32);
    let column_orientation = Orientation::default();
    let row_orientation = Orientation::from_degree(270.0);
    let next_row = Point2d::new(0, size.width() as i32);

    for (row_name, row) in rows.iter() {
        start.x = 0;

        for (column_name, column) in columns.iter() {
            let appearance = create(height, row, column);
            let size = calculate_character_size(&config, &appearance);
            let aabb_front = AABB::new(start, size);

            svg_builder.render_rectangle(&aabb_front, &options);
            render_character_from_front(&mut svg_builder, &config, &aabb_front, &appearance);

            if back_too {
                let start_back = start + next_row;
                let aabb_back = AABB::new(start_back, size);
                svg_builder.render_rectangle(&aabb_back, &options);
                render_character_from_back(&mut svg_builder, &config, &aabb_back, &appearance);
            }

            let text_center = start + column_text_offset;
            svg_builder.render_text(column_name, &text_center, column_orientation, &text_options);

            start.x += size.width() as i32;
        }

        let text_center = Point2d::new(text_size as i32, start.y + size.height() as i32 / 2);
        svg_builder.render_text(row_name, &text_center, row_orientation, &text_options);

        if back_too {
            let text_center = text_center + next_row;
            svg_builder.render_text("Back", &text_center, row_orientation, &text_options);
        }

        start.y += (size.height() * row_size) as i32;
    }

    let svg = svg_builder.finish();
    svg.save(filename).unwrap();
}

pub fn add_names<T: Display>(values: Vec<T>) -> Vec<(String, T)> {
    values
        .into_iter()
        .map(|value| (value.to_string(), value))
        .collect()
}
