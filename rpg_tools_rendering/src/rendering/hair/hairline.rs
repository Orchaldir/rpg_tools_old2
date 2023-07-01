use crate::math::aabb2d::AABB;
use crate::math::point2d::Point2d;
use rpg_tools_core::model::character::appearance::hair::Hairline;
use rpg_tools_core::model::character::appearance::Size;

pub fn add_hairlines(aabb: &AABB, hairline: Hairline, corners: &mut Vec<Point2d>) {
    match hairline {
        Hairline::Round(size) => {
            let hairline_y = get_hairline_y(size);
            add_2_points(corners, aabb, hairline_y, 0.4);
        }
        Hairline::Straight(size) => {
            let hairline_y = get_hairline_y(size);
            add_2_points(corners, aabb, hairline_y, 0.6);
        }
        Hairline::Triangle(size) => {
            let hairline_y = get_hairline_y(size);
            add_2_points(corners, aabb, hairline_y, 0.2);
        }
        Hairline::WidowsPeak(size) => {
            let hairline_y = get_hairline_y(size);
            let (left, right) = aabb.get_mirrored_points(0.4, hairline_y);
            let center = aabb.get_point(0.5, hairline_y + 0.1);

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

fn get_hairline_y(size: Size) -> f32 {
    match size {
        Size::Low => 0.25,
        Size::Medium => 0.2,
        Size::High => 0.15,
    }
}
