use crate::math::aabb2d::AABB;
use crate::math::point2d::Point2d;
use crate::math::polygon2d::Polygon2d;
use crate::renderer::Renderer;
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::character::appearance::eye::brow::shape::EyebrowShape;
use rpg_tools_core::model::character::appearance::eye::brow::style::EyebrowStyle;
use rpg_tools_core::model::character::appearance::eye::brow::EyeBrows;
use rpg_tools_core::model::side::Side;
use rpg_tools_core::model::side::Side::{Left, Right};

pub fn render_eyebrow(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    eyebrow: &EyeBrows,
    aabb: &AABB,
) {
    match eyebrow {
        EyeBrows::Normal {
            color,
            shape,
            style,
        }
        | EyeBrows::Unibrow {
            color,
            shape,
            style,
        } => {
            let options = config.with_thickness(*color, 0.5);
            let polygon = get_normal_eyebrow(config, *shape, *style, aabb, None);
            renderer.render_polygon(&polygon, &options);
        }
        _ => {}
    }
}

pub fn render_eyebrows(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    eyebrow: &EyeBrows,
    left: &AABB,
    right: &AABB,
) {
    match eyebrow {
        EyeBrows::Normal {
            color,
            shape,
            style,
        } => {
            let options = config.with_thickness(*color, 0.5);
            let polygon_left = get_normal_eyebrow(config, *shape, *style, left, Some(Left));
            let polygon_right = get_normal_eyebrow(config, *shape, *style, right, Some(Right));
            renderer.render_polygon(&polygon_left, &options);
            renderer.render_polygon(&polygon_right, &options);
        }
        EyeBrows::Unibrow {
            color,
            shape,
            style,
        } => {}
        _ => {}
    }
}

pub fn get_normal_eyebrow(
    config: &RenderConfig,
    shape: EyebrowShape,
    style: EyebrowStyle,
    aabb: &AABB,
    side: Option<Side>,
) -> Polygon2d {
    Polygon2d::new(vec![Point2d::default(); 3])
}
