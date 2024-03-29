use crate::math::aabb2d::{get_end_x, AABB};
use crate::math::polygon2d::builder::Polygon2dBuilder;
use crate::math::polygon2d::Polygon2d;
use crate::renderer::Renderer;
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::appearance::length::Length;
use rpg_tools_core::model::character::appearance::hair::ponytail::position::PonytailPosition;
use rpg_tools_core::model::character::appearance::hair::ponytail::style::PonytailStyle;
use rpg_tools_core::model::character::appearance::hair::ponytail::Ponytail;
use rpg_tools_core::model::character::appearance::head::HeadShape;

pub fn render_ponytail(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    head_shape: HeadShape,
    ponytail: &Ponytail,
) {
    let options = config.get_options(ponytail.color);
    let radius = 0.2;

    let polygon = match ponytail.position {
        PonytailPosition::High => {
            get_ponytail_down(config, aabb, ponytail.style, radius, ponytail.length)
        }
        PonytailPosition::Low => {
            get_ponytail_down(config, aabb, ponytail.style, 1.0 - radius, ponytail.length)
        }
        _ => get_ponytail_left(
            config,
            aabb,
            head_shape,
            ponytail.style,
            radius,
            ponytail.length,
        ),
    };

    if ponytail.position != PonytailPosition::Right {
        renderer.render_rounded_polygon(&polygon, &options);
    }

    if ponytail.position == PonytailPosition::Right
        || ponytail.position == PonytailPosition::BothSides
    {
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
        PonytailStyle::Braid => get_braid(config, aabb, 0.5, start, length, 0.0),
        PonytailStyle::Bubble => {
            let thin_width = config.hair.ponytail.link_width;
            let thin_length = config.hair.ponytail.link_length;
            let bubble_width = config.hair.ponytail.bubble_width;
            let combined_length = thin_length + bubble_width;
            let n = ((length / combined_length) as u32).max(1);
            let mut y = start;
            let mut builder = Polygon2dBuilder::new();

            for i in 0..n {
                builder.add_mirrored_points(aabb, bubble_width, y, false);
                y += bubble_width;
                builder.add_mirrored_points(aabb, bubble_width, y, false);

                if i < n - 1 {
                    builder.add_mirrored_points(aabb, thin_width, y, false);
                    y += thin_length;
                    builder.add_mirrored_points(aabb, thin_width, y, false);
                }
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
    let x = get_end_x(config.head.get_max_width(head_shape)) + 0.1;
    let bottom_y = start_y + length;

    match style {
        PonytailStyle::Braid => get_braid(
            config,
            aabb,
            start_x + config.hair.ponytail.braid_width / 2.0,
            start_y,
            length,
            config.hair.ponytail.braid_width,
        ),
        PonytailStyle::Bubble => {
            let thin_width = config.hair.ponytail.link_width;
            let thin_length = config.hair.ponytail.link_length;
            let bubble = config.hair.ponytail.bubble_width;
            let bubble_half = bubble / 2.0;
            let combined_length = thin_length + bubble;
            let n = ((length / combined_length) as u32).max(1) - 1;
            let mut x = start_x;
            let mut y = start_y;

            let mut builder = Polygon2dBuilder::new();

            // start link
            builder.add_vertical_pair(aabb, thin_width, x, y, false);
            x += thin_length;
            builder.add_vertical_pair(aabb, thin_width, x, y, false);

            // 1.bubble
            builder.add_vertical_pair(aabb, bubble, x, y, false);
            builder.add_point_cw(aabb.get_point(x + bubble, y - bubble_half), false);
            builder.add_point_cw(aabb.get_point(x + bubble, y + bubble_half), false);
            x += bubble_half;
            y += bubble_half;

            for i in 0..n {
                builder.add_horizontal_pair(aabb, thin_width, x, y, false);

                y += thin_length;

                if i == 0 {
                    x += bubble / 3.0;
                }

                builder.add_horizontal_pair(aabb, thin_width, x, y, false);
                builder.add_horizontal_pair(aabb, bubble, x, y, false);
                y += bubble;
                builder.add_horizontal_pair(aabb, bubble, x, y, false);
            }

            builder.build()
        }
        PonytailStyle::Straight | PonytailStyle::Wide => {
            let width = config.hair.ponytail.width;
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
    }
}

fn get_braid(
    config: &RenderConfig,
    aabb: &AABB,
    start_x: f32,
    start_y: f32,
    length: f32,
    offset: f32,
) -> Polygon2d {
    let braid = config.hair.ponytail.braid_width;
    let half = braid / 2.0;
    let n = ((length / braid) as u32).max(1);
    let mut x = start_x;
    let mut y = start_y;
    let mut step = offset;
    let mut builder = Polygon2dBuilder::new();

    builder.add_horizontal_pair(aabb, braid, x, y, false);

    for _i in 0..n - 1 {
        y += half;
        x += step / 4.0;
        builder.add_point(aabb.get_point(x - half, y), false);
        builder.add_point(aabb.get_point(x, y), false);
        builder.add_point(aabb.get_point(x - half, y), false);
        y += half;
        x += step / 4.0;
        builder.add_point_cw(aabb.get_point(x + half, y), false);
        builder.add_point_cw(aabb.get_point(x, y), false);
        builder.add_point_cw(aabb.get_point(x + half, y), false);
        step /= 2.0;
    }

    y += braid;
    builder.add_point(aabb.get_point(x, y), false);

    builder.build()
}
