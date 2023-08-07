use crate::ui::{UiVisitor, UI};
use macro_ui::ui;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Left or Right? From the [`character's`](crate::model::character::Character) perspective.
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Side {
    Left,
    Right,
}

impl Side {
    pub fn get_all() -> Vec<Side> {
        vec![Side::Left, Side::Right]
    }

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

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<&str> for Side {
    fn from(shape: &str) -> Self {
        match shape {
            "Left" => Self::Left,
            _ => Self::Right,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        for side in Side::get_all() {
            let string = side.to_string();
            assert_eq!(side, Side::from(&*string));
        }
    }
}
