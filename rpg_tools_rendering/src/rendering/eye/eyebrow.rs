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
    center: &Point2d,
    radius: u32,
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
            let polygon = get_normal_eyebrow(config, *shape, *style, center, radius, None);
            renderer.render_polygon(&polygon, &options);
        }
        _ => {}
    }
}

pub fn render_eyebrows(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    eyebrow: &EyeBrows,
    left: &Point2d,
    right: &Point2d,
    radius: u32,
) {
    match eyebrow {
        EyeBrows::Normal {
            color,
            shape,
            style,
        } => {
            let options = config.with_thickness(*color, 0.5);
            let polygon_left = get_normal_eyebrow(config, *shape, *style, left, radius, Some(Left));
            let polygon_right =
                get_normal_eyebrow(config, *shape, *style, right, radius, Some(Right));
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
    center: &Point2d,
    radius: u32,
    side: Option<Side>,
) -> Polygon2d {
    Polygon2d::new(vec![Point2d::default(); 3])
}
