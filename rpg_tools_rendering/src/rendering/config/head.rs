use rpg_tools_core::model::character::appearance::head::RealisticHeadShape::*;
use rpg_tools_core::model::character::appearance::head::{HeadShape, RealisticHeadShape};

/// The rendering config of the [`head`](rpg_tools_core::model::character::appearance::head::Head).
///
/// All the *width* variables are the width of the head at certain levels relative to the head's height.
///
/// All the *y* variables are along the head's height. 0 means the top and 1 the chin.
///
/// # Diagram
///
/// ```svgbob
///
///   +----------> width
///   |
/// 0 |  +-----+ top
///   | /       \
///   | +       + forehead
///   | |       |
///   | +       + eye
///   | |       |
///   | +       + mouth
///   | \       /
/// 1 |  +-----+ chin
///   |
///   v
/// y-axis
/// ```
#[derive(Debug, PartialEq)]
pub struct HeadConfig {
    pub width_wide: f32,
    pub width_narrow: f32,
    pub width_square: f32,
    pub width_round: f32,
    pub width_sharp: f32,
    pub y_forehead: f32,
    pub y_eye: f32,
    pub y_mouth: f32,
}

impl HeadConfig {
    pub fn get_top_width(&self, realistic: RealisticHeadShape) -> f32 {
        match realistic {
            Rectangle | Square => self.width_square,
            _ => self.width_round,
        }
    }

    pub fn get_forehead_width(&self, realistic: RealisticHeadShape) -> f32 {
        match realistic {
            Round | Square | TriangleDown => self.width_wide,
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
            Round | Square => self.width_wide,
            Oval | Rectangle => self.width_narrow,
            _ => (self.width_narrow + self.width_wide) / 2.0,
        }
    }

    pub fn get_mouth_width(&self, realistic: RealisticHeadShape) -> f32 {
        match realistic {
            Round | Square | TriangleUp => self.width_wide,
            _ => self.width_narrow,
        }
    }

    pub fn get_chin_width(&self, realistic: RealisticHeadShape) -> f32 {
        match realistic {
            Rectangle | Square | TriangleUp => self.width_square,
            Round => self.width_round,
            _ => self.width_sharp,
        }
    }
}
