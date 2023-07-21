use crate::renderer::color::WebColor;
use crate::rendering::config::size::SizeConfig;
use rpg_tools_core::model::character::appearance::mouth::TeethColor;
use rpg_tools_core::model::size::Size;

/// The rendering config of the [`mouth`](rpg_tools_core::model::character::appearance::mouth::Mouth).
#[derive(Debug, PartialEq)]
pub struct MouthConfig {
    pub mouth_width: SizeConfig,
    pub distance_between_fangs: f32,
    pub fang_height: SizeConfig,
    pub circular: CircularMouthConfig,
}

impl MouthConfig {
    pub fn get_mouth_width(&self, head_width: f32, size: Size) -> f32 {
        head_width * self.mouth_width.convert(size)
    }

    pub fn get_distance_between_fangs(&self, mouth_width: f32) -> f32 {
        mouth_width * self.distance_between_fangs
    }

    pub fn get_fang_height(&self, head_height: u32, size: Size) -> f32 {
        head_height as f32 * self.fang_height.convert(size)
    }

    pub fn get_teeth_color(&self, color: TeethColor) -> WebColor {
        match color {
            TeethColor::White => WebColor::from_rgb(255, 255, 255),
            TeethColor::Yellow => WebColor::from_rgb(249, 232, 158),
            TeethColor::Brown => WebColor::Name("brown".to_string()),
        }
    }
}

/// The rendering config of a [`circular mouth`](rpg_tools_core::model::character::appearance::mouth::Mouth::Circle).
#[derive(Debug, PartialEq)]
pub struct CircularMouthConfig {
    pub radius: SizeConfig,
    pub fang_height: f32,
}

impl CircularMouthConfig {
    pub fn get_mouth_radius(&self, head_width: u32, size: Size) -> u32 {
        (head_width as f32 * self.radius.convert(size)) as u32
    }

    pub fn get_fang_height(&self, radius: u32) -> f32 {
        self.fang_height * radius as f32
    }
}
