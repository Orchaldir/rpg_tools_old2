use rpg_tools_core::model::character::appearance::head::RealisticHeadShape::*;
use rpg_tools_core::model::character::appearance::head::{
    GeometricHeadShape, HeadShape, RealisticHeadShape,
};

/// The rendering config of the [`head`](rpg_tools_core::model::character::appearance::head::Head).
///
/// All the *width* variables are the width of the head at certain levels relative to the head's height.
/// 1 means 100% of the height.
///
/// All the *y* variables are along the head's height. 0 means the top and 1 the chin.
///
/// # Diagram
///
/// ```svgbob
///
///   +----------> width
///   |
/// 0 |  +-----+  top
///   | /       \
///   | +       + forehead
///   | |       |
///   | +       + eye
///   | |       |
///   | +       + mouth
///   | \       /
/// 1 |  +-----+  chin
///   |
///   v
/// y-axis
/// ```
#[derive(Debug, PartialEq)]
pub struct HeadConfig {
    /// The width at forehead, eye or mouth level, if wide.
    pub width_wide: f32,
    /// The width at forehead, eye or mouth level, if narrow.
    pub width_narrow: f32,
    /// The width at top & chin level, if that end is square.
    pub width_square: f32,
    /// The width at top & chin level, if that end is round.
    pub width_round: f32,
    /// The width at top & chin level, if that end is sharp.
    pub width_sharp: f32,
    pub y_forehead: f32,
    pub y_eye: f32,
    pub y_mouth: f32,
}

impl HeadConfig {
    pub fn get_top_width(&self, realistic: RealisticHeadShape) -> f32 {
        match realistic {
            RoundedRectangle | RoundedSquare => self.width_square,
            _ => self.width_round,
        }
    }

    pub fn get_forehead_width(&self, realistic: RealisticHeadShape) -> f32 {
        match realistic {
            Round | RoundedSquare | TriangleDown => self.width_wide,
            _ => self.width_narrow,
        }
    }

    pub fn get_eye_width(&self, shape: HeadShape) -> f32 {
        match shape {
            HeadShape::Geometric(_) => 1.0,
            HeadShape::Realistic(realistic) => self.get_eye_width_realistic(realistic),
        }
    }

    pub fn get_eye_width_realistic(&self, realistic: RealisticHeadShape) -> f32 {
        match realistic {
            Round | RoundedSquare => self.width_wide,
            Oval | RoundedRectangle => self.width_narrow,
            _ => (self.width_narrow + self.width_wide) / 2.0,
        }
    }

    pub fn get_mouth_width(&self, shape: HeadShape) -> f32 {
        match shape {
            HeadShape::Geometric(geometric) => self.get_mouth_width_geometric(geometric),
            HeadShape::Realistic(realistic) => self.get_mouth_width_realistic(realistic),
        }
    }

    pub fn get_mouth_width_geometric(&self, geometric: GeometricHeadShape) -> f32 {
        match geometric {
            GeometricHeadShape::Circle => 1.0,
            GeometricHeadShape::Square => get_circle_width(self.y_mouth),
        }
    }

    pub fn get_mouth_width_realistic(&self, realistic: RealisticHeadShape) -> f32 {
        match realistic {
            Round | RoundedSquare | TriangleUp => self.width_wide,
            _ => self.width_narrow,
        }
    }

    pub fn get_chin_width(&self, realistic: RealisticHeadShape) -> f32 {
        match realistic {
            RoundedRectangle | RoundedSquare | TriangleUp => self.width_square,
            Round => self.width_round,
            _ => self.width_sharp,
        }
    }
}

/// Calculates the width of the circle at a certain y position.
///
/// ```
///# use rpg_tools_rendering::rendering::config::head::get_circle_width;
/// assert_eq!(get_circle_width(0.0), 0.0);
/// assert_eq!(get_circle_width(0.25), 0.8660254);
/// assert_eq!(get_circle_width(0.5), 1.0);
/// assert_eq!(get_circle_width(0.75), 0.8660254);
/// assert_eq!(get_circle_width(1.0), 0.0);
/// ```
pub fn get_circle_width(y: f32) -> f32 {
    (0.25 - (y - 0.5).powi(2)).sqrt() * 2.0
}
