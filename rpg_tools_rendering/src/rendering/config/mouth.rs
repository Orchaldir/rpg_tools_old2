use crate::renderer::color::WebColor;
use rpg_tools_core::model::character::appearance::mouth::TeethColor;
use rpg_tools_core::model::character::appearance::Size;

/// The rendering config of the [`mouth`](rpg_tools_core::model::character::appearance::mouth::Mouth).
#[derive(Debug, PartialEq)]
pub struct MouthConfig {
    pub mouth_width_low: f32,
    pub mouth_width_medium: f32,
    pub mouth_width_high: f32,
    pub distance_between_fangs: f32,
    pub fang_height_low: f32,
    pub fang_height_medium: f32,
    pub fang_height_high: f32,
}

impl MouthConfig {
    pub fn get_mouth_width(&self, head_width: f32, size: Size) -> f32 {
        head_width
            * match size {
                Size::Low => self.mouth_width_low,
                Size::Medium => self.mouth_width_medium,
                Size::High => self.mouth_width_high,
            }
    }

    pub fn get_distance_between_fangs(&self, mouth_width: f32) -> f32 {
        mouth_width * self.distance_between_fangs
    }

    pub fn get_fang_height(&self, head_height: u32, size: Size) -> f32 {
        head_height as f32
            * match size {
                Size::Low => self.fang_height_low,
                Size::Medium => self.fang_height_medium,
                Size::High => self.fang_height_high,
            }
    }

    pub fn get_teeth_color(&self, color: TeethColor) -> WebColor {
        match color {
            TeethColor::White => WebColor::from_rgb(255, 255, 255),
            TeethColor::Yellow => WebColor::from_rgb(249, 232, 158),
            TeethColor::Brown => WebColor::Name("brown".to_string()),
            TeethColor::Black => WebColor::from_rgb(0, 0, 0),
        }
    }
}
