use crate::rendering::config::width::WidthConfig;
use rpg_tools_core::model::character::appearance::body::Body;

/// The rendering config of the [`body`](rpg_tools_core::model::character::appearance::body::Body).
#[derive(Debug, PartialEq)]
pub struct BodyConfig {
    pub width: WidthConfig,
}

impl BodyConfig {
    pub fn get_width_factor(&self, body: &Body) -> f32 {
        self.width.convert(body.width)
    }
}
