use crate::math::aabb2d::AABB;
use crate::math::point2d::Point2d;
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::character::appearance::hair::hairline::Hairline;

pub fn add_hairlines(
    config: &RenderConfig,
    aabb: &AABB,
    hairline: Hairline,
    corners: &mut Vec<Point2d>,
) {
    let hairline_y = config.hair.hairline.y.convert(hairline.get_y_position());

    match hairline {
        Hairline::Round { .. } => {
            add_2_points(corners, aabb, hairline_y, config.hair.hairline.width_round);
        }
        Hairline::Straight { .. } => {
            add_2_points(
                corners,
                aabb,
                hairline_y,
                config.hair.hairline.width_straight,
            );
        }
        Hairline::Triangle { .. } => {
            add_2_points(
                corners,
                aabb,
                hairline_y,
                config.hair.hairline.width_triangle,
            );
        }
        Hairline::WidowsPeak { .. } => {
            let (left, right) =
                aabb.get_mirrored_points(config.hair.hairline.width_widows_peak, hairline_y);
            let center = aabb.get_point(0.5, hairline_y + config.hair.hairline.height_widows_peak);

            corners.push(left);
            corners.push(center);
            corners.push(right);
        }
    }
}

fn add_2_points(corners: &mut Vec<Point2d>, aabb: &AABB, y: f32, width: f32) {
    let (left, right) = aabb.get_mirrored_points(width, y);

    corners.push(left);
    corners.push(right);
}
