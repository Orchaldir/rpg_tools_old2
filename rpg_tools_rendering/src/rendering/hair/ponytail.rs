use crate::math::aabb2d::{get_end_x, AABB};
use crate::math::polygon2d::builder::Polygon2dBuilder;
use crate::math::polygon2d::Polygon2d;
use crate::renderer::Renderer;
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::character::appearance::hair::ponytail::position::PonytailPosition;
use rpg_tools_core::model::character::appearance::hair::ponytail::style::PonytailStyle;
use rpg_tools_core::model::character::appearance::head::HeadShape;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;

pub fn render_ponytail(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    head_shape: HeadShape,
    position: PonytailPosition,
    style: PonytailStyle,
    length: Length,
    color: Color,
) {
    let options = config.get_options(color);
    let radius = 0.2;

    let polygon = match position {
        PonytailPosition::High => get_ponytail_down(config, aabb, style, radius, length),
        PonytailPosition::Low => get_ponytail_down(config, aabb, style, 1.0 - radius, length),
        _ => get_ponytail_left(config, aabb, head_shape, style, radius, length),
    };

    if position != PonytailPosition::Right {
        renderer.render_rounded_polygon(&polygon, &options);
    }

    if position == PonytailPosition::Right || position == PonytailPosition::BothSides {
        let right = aabb.mirrored(&polygon);
        renderer.render_rounded_polygon(&right, &options);
    }
}

fn get_ponytail_down(
    config: &RenderConfig,
    aabb: &AABB,
    style: PonytailStyle,
    start: f32,
    length: Length,
) -> Polygon2d {
    let length = aabb.convert_from_height(length.to_millimetre());

    match style {
        PonytailStyle::Bubble => {
            let thin_width = config.hair.ponytail.link_width;
            let thin_length = config.hair.ponytail.link_length;
            let bubble_width = config.hair.ponytail.bubble_width;
            let combined_length = thin_length + bubble_width;
            let n = ((length / combined_length) as u32).max(1);
            let mut y = start;

            let mut builder = Polygon2dBuilder::new();

            for _i in 0..n {
                builder.add_mirrored_points(aabb, bubble_width, y, false);
                y += bubble_width;
                builder.add_mirrored_points(aabb, bubble_width, y, false);
                builder.add_mirrored_points(aabb, thin_width, y, false);
                y += thin_length;
                builder.add_mirrored_points(aabb, thin_width, y, false);
            }

            builder.build()
        }
        PonytailStyle::Straight | PonytailStyle::Wide => {
            let width = config.hair.ponytail.width;
            let bottom_width = config.hair.ponytail.get_bottom_width(style);
            let (top_left, top_right) = aabb.get_mirrored_points(width, start);
            let (bottom_left, bottom_right) =
                aabb.get_mirrored_points(bottom_width, start + length);

            let corners = vec![top_left, bottom_left, bottom_right, top_right];

            Polygon2d::new(corners)
        }
    }
}

fn get_ponytail_left(
    config: &RenderConfig,
    aabb: &AABB,
    head_shape: HeadShape,
    style: PonytailStyle,
    start_y: f32,
    length: Length,
) -> Polygon2d {
    let length = aabb.convert_from_height(length.to_millimetre());
    let start_head_width =
        (config.head.get_top_width(head_shape) + config.head.get_forehead_width(head_shape)) / 2.0;
    let start_x = get_end_x(start_head_width);
    let width = config.hair.ponytail.width;
    let x = get_end_x(config.head.get_max_width(head_shape)) + 0.1;
    let bottom_y = start_y + length;
    let start_half = width / 2.0;
    let bottom_width = config.hair.ponytail.get_bottom_width(style);

    let center_top = aabb.get_point(start_x - start_half, start_y - start_half);
    let center_bottom = aabb.get_point(start_x - start_half, start_y + start_half);
    let top_left = aabb.get_point(x, start_y + start_half);
    let top_right = aabb.get_point(x + width, start_y - start_half);
    let bottom_left = aabb.get_point(x, bottom_y);
    let bottom_right = aabb.get_point(x + bottom_width, bottom_y);

    let corners = vec![
        center_top,
        center_bottom,
        top_left,
        bottom_left,
        bottom_right,
        top_right,
    ];

    Polygon2d::new(corners)
}
