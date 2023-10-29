use crate::model::color::Color;
use crate::model::side::Side;
use macro_convert::Convert;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

/// All devices worn over a [`character's`](crate::model::character::Character)
/// [`eye(s)`](crate::model::character::appearance::eye::Eyes).
#[derive(ui, Clone, Copy, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Eyewear {
    #[default]
    None,
    Glasses {
        style: LensStyle,
    },
    Monocle {
        style: LensStyle,
        side: Side,
    },
}

/// The style of a [`eyewear's`](Eyewear) lens.
#[derive(ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct LensStyle {
    pub frame_color: Color,
    pub frame_type: FrameType,
    pub lens_color: Color,
    pub lens_shape: LensShape,
}

/// The frame of the [`eyewear's`](Eyewear).
#[derive(Convert, ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum FrameType {
    #[default]
    FullRimmed,
    SemiRimless,
    Rimless,
    Wire,
}

/// The shape of a [`eyewear's`](Eyewear) lens.
#[derive(Convert, ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum LensShape {
    Circle,
    #[default]
    Oval,
    Rectangle,
}
