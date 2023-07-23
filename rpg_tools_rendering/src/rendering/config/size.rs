use rpg_tools_core::model::size::Size;

/// The configuration of a [`size`](rpg_tools_core::model::size::Size).
#[derive(Debug, PartialEq)]
pub struct SizeConfig {
    pub low: f32,
    pub medium: f32,
    pub high: f32,
}

impl SizeConfig {
    pub fn convert(&self, size: Size) -> f32 {
        match size {
            Size::Small => self.low,
            Size::Medium => self.medium,
            Size::Large => self.high,
        }
    }
}
