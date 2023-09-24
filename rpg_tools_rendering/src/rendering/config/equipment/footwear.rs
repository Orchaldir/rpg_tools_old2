use crate::rendering::config::body::BodyConfig;
use rpg_tools_core::model::character::appearance::body::Body;
use rpg_tools_core::model::equipment::appearance::footwear::FootwearStyle;

/// The rendering config of the [`footwear`](rpg_tools_core::model::equipment::appearance::footwear::Footwear).
#[derive(Debug, PartialEq)]
pub struct FootwearConfig {
    pub height_ankle: f32,
    pub height_knee: f32,
    pub height_sole: f32,
    pub width_shaft: f32,
    pub width_sole: f32,
}

impl FootwearConfig {
    pub fn get_shaft_y(
        &self,
        config: &BodyConfig,
        body: &Body,
        style: FootwearStyle,
    ) -> Option<f32> {
        match style {
            FootwearStyle::KneeHighBoots => Some(self.to_y(config, body, self.height_knee)),
            _ => None,
        }
    }

    pub fn to_y(&self, config: &BodyConfig, body: &Body, height: f32) -> f32 {
        config.y_foot - height - config.get_foot_radius_factor(body)
    }
}
