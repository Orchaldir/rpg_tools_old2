use rpg_tools_core::model::character::appearance::head::RealisticHeadShape;
use rpg_tools_core::model::character::appearance::head::RealisticHeadShape::*;

#[derive(Debug, PartialEq)]
pub struct HeadConfig {
    pub wide_width: f32,
    pub narrow_width: f32,
}

impl HeadConfig {
    pub fn get_top_width(&self, realistic: RealisticHeadShape) -> f32 {
        match realistic {
            Rectangle | Square => 0.3,
            _ => 0.2,
        }
    }

    pub fn get_forehead_width(&self, realistic: RealisticHeadShape) -> f32 {
        match realistic {
            Round | Square | TriangleDown => self.wide_width,
            _ => self.narrow_width,
        }
    }

    pub fn get_mouth_width(&self, realistic: RealisticHeadShape) -> f32 {
        match realistic {
            Round | Square | TriangleUp => self.wide_width,
            _ => self.narrow_width,
        }
    }

    pub fn get_chin_width(&self, realistic: RealisticHeadShape) -> f32 {
        match realistic {
            Rectangle | Square | TriangleUp => 0.3,
            Round => 0.2,
            _ => 0.1,
        }
    }
}
