use crate::math::aabb2d::AABB;
use crate::math::polygon2d::Polygon2d;
use crate::renderer::Renderer;
use crate::rendering::beard::full::{
    get_full_forked, get_full_rectangle, get_full_triangle, get_full_wide,
};
use crate::rendering::beard::goatee::{get_goat_patch, get_goatee, get_soul_patch, get_van_dyke};
use crate::rendering::beard::moustache::*;
use crate::rendering::config::RenderConfig;
use crate::rendering::head::render_head_shape_with_option;
use rpg_tools_core::model::character::appearance::beard::full::FullBeardStyle;
use rpg_tools_core::model::character::appearance::beard::goatee::GoateeStyle;
use rpg_tools_core::model::character::appearance::beard::moustache::MoustacheStyle;
use rpg_tools_core::model::character::appearance::beard::Beard;
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape};
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;

pub mod full;
pub mod goatee;
pub mod moustache;

pub fn render_beard_behind_mouth(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    head_shape: HeadShape,
    head: &Head,
    beard: &Beard,
) {
    match beard {
        Beard::Stubble { color } => {
            render_stubble(renderer, config, aabb, head, *color);
        }
        Beard::FullBeard {
            style,
            length,
            color,
        } => {
            render_full_beard(renderer, config, aabb, head_shape, style, length, color);
        }
        _ => {}
    }
}

pub fn render_beard_in_front_of_mouth(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    beard: &Beard,
    mouth_width: f32,
) {
    match beard {
        Beard::Moustache { moustache, color } => {
            render_mustache(renderer, config, aabb, mouth_width, moustache, color);
        }
        Beard::Goatee { goatee, color } => {
            render_goatee(renderer, config, aabb, mouth_width, goatee, color);
        }
        Beard::GoateeAndMoustache {
            moustache,
            goatee,
            color,
        } => {
            render_mustache(renderer, config, aabb, mouth_width, moustache, color);
            render_goatee(renderer, config, aabb, mouth_width, goatee, color);
        }
        _ => {}
    }
}

fn render_goatee(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    mouth_width: f32,
    style: &GoateeStyle,
    color: &Color,
) {
    let options = config.without_line(*color);
    let polygon = match style {
        GoateeStyle::GoatPatch => get_goat_patch(config, aabb, mouth_width),
        GoateeStyle::Goatee => get_goatee(config, aabb, mouth_width),
        GoateeStyle::VanDyke => get_van_dyke(config, aabb),
        _ => get_soul_patch(config, aabb),
    };
    renderer.render_polygon(&polygon, &options);
}

fn render_full_beard(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    head_shape: HeadShape,
    style: &FullBeardStyle,
    length: &Length,
    color: &Color,
) {
    let options = config.get_options(*color);
    let polygon = match style {
        FullBeardStyle::Fork => get_full_forked(config, aabb, head_shape, length),
        FullBeardStyle::Rectangle => get_full_rectangle(config, aabb, head_shape, length),
        FullBeardStyle::Triangle => get_full_triangle(config, aabb, head_shape, length),
        FullBeardStyle::Wide => get_full_wide(config, aabb, head_shape, length),
    };

    renderer.render_polygon(&polygon, &options);
}

fn render_mustache(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    mouth_width: f32,
    style: &MoustacheStyle,
    color: &Color,
) {
    let options = config.without_line(*color);
    let polygon = match style {
        MoustacheStyle::FuManchu => get_fu_manchu(config, aabb, mouth_width),
        MoustacheStyle::Handlebar => get_handlebar(config, aabb, mouth_width),
        MoustacheStyle::Pencil => get_pencil(config, aabb, mouth_width),
        MoustacheStyle::Pyramid => get_pyramid(config, aabb, mouth_width),
        MoustacheStyle::Toothbrush => get_toothbrush(config, aabb),
        MoustacheStyle::Walrus => get_walrus(config, aabb, mouth_width),
    };
    renderer.render_polygon(&polygon, &options);
}

fn render_stubble(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    head: &Head,
    color: Color,
) {
    let options = config.without_line(color);
    let line = config.get_line_options(1.0);
    let polygon = get_stubble(config, aabb, head.shape);

    renderer.render_polygon(&polygon, &options);
    render_head_shape_with_option(renderer, config, aabb, line, head.shape);
}

fn get_stubble(config: &RenderConfig, aabb: &AABB, head_shape: HeadShape) -> Polygon2d {
    let top_width =
        (config.head.get_eye_width(head_shape) + config.head.get_mouth_width(head_shape)) / 2.0;
    let top_y = config.head.get_moustache_y();
    let (top_left, top_right) = aabb.get_mirrored_points(top_width, top_y);
    let (bottom_left, bottom_right) =
        aabb.get_mirrored_points(config.head.get_chin_width(head_shape), 1.0);
    let corners = vec![top_left, bottom_left, bottom_right, top_right];

    let polygon = Polygon2d::new(corners);
    config.cut_corners(&polygon).unwrap()
}
