use macro_convert::Convert;
use serde::{Deserialize, Serialize};

/// The relationship types between 2 [`characters`](crate::model::character::Character)
/// that are romantic.
#[derive(Convert, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum RomanticRelationship {
    ExLover,
    ExSpouse,
    Lover,
    Spouse,
}
