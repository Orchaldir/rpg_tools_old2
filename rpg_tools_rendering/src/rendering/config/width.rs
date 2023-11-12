use rpg_tools_core::model::appearance::width::Width;
use Width::*;

/// The configuration of a [`width`](Width).
#[derive(Debug, PartialEq)]
pub struct WidthConfig {
    pub thin: f32,
    pub average: f32,
    pub wide: f32,
}

impl WidthConfig {
    pub fn convert(&self, width: Width) -> f32 {
        match width {
            Thin => self.thin,
            Average => self.average,
            Wide => self.wide,
        }
    }
}
