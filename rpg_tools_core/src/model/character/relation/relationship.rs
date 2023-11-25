use macro_convert::Convert;
use serde::{Deserialize, Serialize};

pub const RELATIONSHIPS_FILE: &str = "relations/relationships.csv";

/// The relationship types between 2 [`characters`](crate::model::character::Character)
/// that are not romantic or between family.
#[derive(Convert, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Relationship {
    Enemy,
    Friend,
    Rival,
}
