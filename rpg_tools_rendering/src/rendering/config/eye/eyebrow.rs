use crate::rendering::config::width::WidthConfig;
use rpg_tools_core::model::character::appearance::eye::brow::style::EyebrowStyle;
use rpg_tools_core::model::side::Side;

/// The rendering config of the [`eyebrows`](rpg_tools_core::model::character::appearance::eye::brow::EyeBrows).
#[derive(Debug, PartialEq)]
pub struct EyebrowConfig {
    pub width: WidthConfig,
    pub distance_to_eye: f32,
    pub distance_to_eye_straight: f32,
}

impl EyebrowConfig {
    /// The eyebrow thickness closer to the center of the face.
    pub fn get_inner_thickness(&self, style: EyebrowStyle) -> f32 {
        match style {
            EyebrowStyle::Bushy | EyebrowStyle::Winged => self.width.wide,
            EyebrowStyle::Even => self.width.average,
            EyebrowStyle::Thin | EyebrowStyle::Managerial => self.width.thin,
        }
    }

    /// The eyebrow thickness closer to the side of the face.
    pub fn get_outer_thickness(&self, style: EyebrowStyle) -> f32 {
        match style {
            EyebrowStyle::Bushy | EyebrowStyle::Managerial => self.width.wide,
            EyebrowStyle::Even => self.width.average / 2.0,
            EyebrowStyle::Thin | EyebrowStyle::Winged => self.width.thin / 2.0,
        }
    }

    /// The eyebrow thickness on the left side of the eye.
    pub fn get_left_thickness(&self, style: EyebrowStyle, side: Option<Side>) -> f32 {
        match side {
            None => self.get_center_thickness(style),
            Some(side) => match side {
                Side::Left => self.get_outer_thickness(style),
                Side::Right => self.get_inner_thickness(style),
            },
        }
    }

    /// The eyebrow thickness on the right side of the eye.
    pub fn get_right_thickness(&self, style: EyebrowStyle, side: Option<Side>) -> f32 {
        match side {
            None => self.get_center_thickness(style),
            Some(side) => match side {
                Side::Left => self.get_inner_thickness(style),
                Side::Right => self.get_outer_thickness(style),
            },
        }
    }

    /// The eyebrow thickness in the center.
    pub fn get_center_thickness(&self, style: EyebrowStyle) -> f32 {
        match style {
            EyebrowStyle::Bushy => self.width.wide,
            EyebrowStyle::Thin => self.width.thin,
            _ => self.width.average,
        }
    }
}
