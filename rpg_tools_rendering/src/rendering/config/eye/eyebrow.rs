use crate::rendering::config::width::WidthConfig;
use rpg_tools_core::model::character::appearance::eye::brow::style::EyebrowStyle;
use rpg_tools_core::model::side::Side;
use rpg_tools_core::model::width::Width;

/// The rendering config of the [`eyebrows`](rpg_tools_core::model::character::appearance::eye::brow::EyeBrows).
#[derive(Debug, PartialEq)]
pub struct EyebrowConfig {
    pub width: WidthConfig,
    pub relative_width: WidthConfig,
    pub distance_to_eye: f32,
    pub distance_to_eye_straight: f32,
}

impl EyebrowConfig {
    /// The eyebrow thickness closer to the center of the face.
    fn get_inner_thickness(&self, style: EyebrowStyle) -> f32 {
        match style {
            EyebrowStyle::Winged => self.relative_width.wide,
            EyebrowStyle::Even => self.relative_width.average,
            EyebrowStyle::Managerial => self.relative_width.thin,
        }
    }

    /// The eyebrow thickness closer to the side of the face.
    fn get_outer_thickness(&self, style: EyebrowStyle) -> f32 {
        match style {
            EyebrowStyle::Managerial => self.relative_width.wide,
            EyebrowStyle::Even => self.relative_width.average / 2.0,
            EyebrowStyle::Winged => self.relative_width.thin / 2.0,
        }
    }

    /// The eyebrow thickness on the left side of the eye.
    pub fn get_left_thickness(&self, style: EyebrowStyle, width: Width, side: Option<Side>) -> f32 {
        self.width.convert(width)
            * match side {
                None => 1.0,
                Some(side) => match side {
                    Side::Left => self.get_outer_thickness(style),
                    Side::Right => self.get_inner_thickness(style),
                },
            }
    }

    /// The eyebrow thickness on the right side of the eye.
    pub fn get_right_thickness(
        &self,
        style: EyebrowStyle,
        width: Width,
        side: Option<Side>,
    ) -> f32 {
        self.width.convert(width)
            * match side {
                None => 1.0,
                Some(side) => match side {
                    Side::Left => self.get_inner_thickness(style),
                    Side::Right => self.get_outer_thickness(style),
                },
            }
    }

    /// The eyebrow thickness in the center.
    pub fn get_center_thickness(&self, width: Width) -> f32 {
        self.width.convert(width)
    }
}
