use rpg_tools_core::model::character::appearance::hair::ponytail::style::PonytailStyle;

#[derive(Debug, PartialEq)]
pub struct PonytailConfig {
    pub width: f32,
    pub wide_width: f32,
    pub braid_width: f32,
    pub bubble_width: f32,
    pub link_width: f32,
    pub link_length: f32,
}

impl PonytailConfig {
    pub fn get_bottom_width(&self, style: PonytailStyle) -> f32 {
        if style == PonytailStyle::Wide {
            self.wide_width
        } else {
            self.width
        }
    }
}
