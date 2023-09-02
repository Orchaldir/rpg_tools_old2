use crate::math::aabb2d::AABB;
use crate::renderer::Renderer;
use crate::rendering::config::RenderConfig;
use crate::rendering::hair::bun::render_buns;
use crate::rendering::hair::short::{
    get_flat_top_back, get_flat_top_front, get_middle_part, get_side_part,
    get_simple_hair_style_polyon, render_buzz_cut,
};
use crate::rendering::head::render_head_shape_with_option;
use crate::rendering::render_polygon;
use rpg_tools_core::model::character::appearance::hair::{Hair, ShortHair};
use rpg_tools_core::model::character::appearance::head::Head;

pub mod bun;
pub mod hairline;
pub mod short;

pub fn render_hair_before_head_front(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    head: &Head,
) {
    match head.hair {
        Hair::None => {}
        Hair::Short {
            style,
            hairline,
            color,
        } => match style {
            ShortHair::BuzzCut => {
                render_buzz_cut(renderer, config, aabb, head.shape, hairline, color)
            }
            ShortHair::FlatTop(size) => {
                let polygon = get_flat_top_front(config, aabb, head.shape, hairline, size);
                render_polygon(renderer, config, &polygon, color);
            }
            ShortHair::MiddlePart => {
                let polygon = get_middle_part(config, aabb, head.shape, hairline);
                render_polygon(renderer, config, &polygon, color);
            }
            ShortHair::SidePart(side) => {
                let polygon = get_side_part(config, aabb, head.shape, side);
                render_polygon(renderer, config, &polygon, color);
            }
        },
        Hair::Long { .. } => {}
        Hair::Bun {
            hairline, color, ..
        } => {
            let polygon = get_simple_hair_style_polyon(config, aabb, head.shape, hairline);
            render_polygon(renderer, config, &polygon, color);
        }
    }
}

pub fn render_hair_behind_head_front(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    head: &Head,
) {
    if let Hair::Bun {
        style, size, color, ..
    } = head.hair
    {
        render_buns(renderer, config, aabb, head.shape, style, size, color);
    }
}

pub fn render_hair_back(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    head: &Head,
) {
    match head.hair {
        Hair::None => {}
        Hair::Short { style, color, .. } => match style {
            ShortHair::FlatTop(size) => {
                let polygon = get_flat_top_back(config, aabb, head.shape, size);
                render_polygon(renderer, config, &polygon, color);
            }
            _ => {
                let options = config.get_options(color);
                render_head_shape_with_option(renderer, config, aabb, options, head.shape);
            }
        },
        Hair::Long { .. } => {}
        Hair::Bun {
            style, size, color, ..
        } => {
            let options = config.get_options(color);
            render_head_shape_with_option(renderer, config, aabb, options, head.shape);
            render_buns(renderer, config, aabb, head.shape, style, size, color);
        }
    }
}
