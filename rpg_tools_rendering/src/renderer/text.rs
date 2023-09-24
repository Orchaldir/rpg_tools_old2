use crate::renderer::color::WebColor;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextOptions {
    pub color: WebColor,
    pub size: u32,
}

impl TextOptions {
    pub const fn new(color: WebColor, size: u32) -> Self {
        Self { color, size }
    }
}
