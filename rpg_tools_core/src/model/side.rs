use crate::ui::{UiVisitor, UI};
use serde::Serialize;
use ui_macro::ui;

/// Left or Right? From the [`character's`](crate::model::character::Character) perspective.
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Side {
    Left,
    Right,
}

impl Side {
    /// Get the sign along the x-axis if viewed from the front.
    /// The right side is on the left and so the sign is negative.
    ///
    /// ```
    ///# use rpg_tools_core::model::side::Side;
    /// assert_eq!(Side::Left.get_sign_from_front(), 1.0);
    /// assert_eq!(Side::Right.get_sign_from_front(), -1.0);
    /// ```
    pub fn get_sign_from_front(&self) -> f32 {
        match self {
            Side::Left => 1.0,
            Side::Right => -1.0,
        }
    }
}
