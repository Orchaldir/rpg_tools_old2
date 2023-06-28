use crate::math::aabb2d::AABB;
use crate::math::polygon2d::Polygon2d;
use crate::renderer::{RenderOptions, Renderer};
use crate::rendering::config::RenderConfig;
use crate::rendering::head::render_realistic_with_option;
use rpg_tools_core::model::character::appearance::hair::{Hair, HairColor, ShortHair};
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape, RealisticHeadShape};

pub fn render_hair(renderer: &mut dyn Renderer, config: &RenderConfig, aabb: &AABB, head: &Head) {
    match head.shape {
        HeadShape::Geometric(_) => {}
        HeadShape::Realistic(realistic) => match head.hair {
            Hair::None => {}
            Hair::Short { style, color } => match style {
                ShortHair::BuzzCut => {
                    render_buzz_cut_realistic(renderer, config, aabb, realistic, color)
                }
                ShortHair::CrewCut => {
                    render_crew_cut_realistic(renderer, config, aabb, realistic, color)
                }
                ShortHair::SidePart(_) => {}
            },
        },
    }
}

fn render_buzz_cut_realistic(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    realistic: RealisticHeadShape,
    color: HairColor,
) {
    let options = RenderOptions::no_line(config.get_hair_color(color));
    let line = config.get_line_options(1.0);
    let polygon = get_cut_realistic(config, aabb, realistic);

    renderer.render_polygon(&polygon, &options);
    render_realistic_with_option(renderer, config, aabb, line, realistic);
}

fn render_crew_cut_realistic(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    realistic: RealisticHeadShape,
    color: HairColor,
) {
    let options = config.get_hair_options(color);
    let mut polygon = get_cut_realistic(config, aabb, realistic);
    polygon = polygon.resize(1.03);
    renderer.render_polygon(&polygon, &options);
}

fn get_cut_realistic(
    config: &RenderConfig,
    aabb: &AABB,
    realistic: RealisticHeadShape,
) -> Polygon2d {
    let (top_left, top_right) = aabb.get_mirrored_points(config.head.get_top_width(realistic), 0.0);
    let (forehead_left, forehead_right) = aabb.get_mirrored_points(
        config.head.get_forehead_width(realistic),
        config.head.y_forehead,
    );
    let (bottom_left, bottom_right) = aabb.get_mirrored_points(
        config.head.get_eye_width_realistic(realistic),
        config.head.y_eye,
    );
    let bottom = aabb.get_point(0.5, 0.1);
    let polygon = Polygon2d::new(vec![
        top_left,
        forehead_left,
        bottom_left,
        bottom,
        bottom_right,
        forehead_right,
        top_right,
    ]);
    config.cut_corners(&polygon).unwrap()
}
