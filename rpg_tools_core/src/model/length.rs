use crate::ui::{UiParser, UiVisitor, UI};
use macro_ui::ui;
use serde::{Deserialize, Serialize};

const METRE_FACTOR: f32 = 1000.0;
const CENTIMETRE_FACTOR: f32 = 10.0;

/// A length or distance. The internal unit is millimetre.
#[derive(ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Length {
    millimetre: u32,
}

impl Length {
    /// Creates a length from meters.
    ///
    /// ```
    ///# use rpg_tools_core::model::length::Length;
    ///
    /// assert_eq!(Length::from_millimetre(1500), Length::from_metre(1.5));
    /// ```
    pub fn from_metre(metre: f32) -> Self {
        Self {
            millimetre: (metre * METRE_FACTOR) as u32,
        }
    }

    /// Creates a length from centimetres.
    ///
    /// ```
    ///# use rpg_tools_core::model::length::Length;
    ///
    /// assert_eq!(Length::from_millimetre(15), Length::from_centimetre(1.5));
    /// ```
    pub fn from_centimetre(metre: f32) -> Self {
        Self {
            millimetre: (metre * CENTIMETRE_FACTOR) as u32,
        }
    }

    /// Creates a length from millimetres.
    pub fn from_millimetre(millimetre: u32) -> Self {
        Self { millimetre }
    }

    /// Converts to metres.
    ///
    /// ```
    ///# use rpg_tools_core::model::length::Length;
    ///
    /// assert_eq!(Length::from_millimetre(1500).to_metre(), 1.5);
    /// ```
    pub fn to_metre(&self) -> f32 {
        self.millimetre as f32 / METRE_FACTOR
    }

    /// Converts to centimetres.
    ///
    /// ```
    ///# use rpg_tools_core::model::length::Length;
    ///
    /// assert_eq!(Length::from_millimetre(15).to_centimetre(), 1.5);
    /// ```
    pub fn to_centimetre(&self) -> f32 {
        self.millimetre as f32 / CENTIMETRE_FACTOR
    }

    /// Converts to millimetres.
    ///
    /// ```
    ///# use rpg_tools_core::model::length::Length;
    ///
    /// assert_eq!(Length::from_metre(0.5).to_millimetre(), 500);
    /// ```
    pub fn to_millimetre(&self) -> u32 {
        self.millimetre
    }
}
