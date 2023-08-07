use macro_convert::Convert;
use serde::{Deserialize, Serialize};

/// The gender of the [`character`](crate::model::character::Character).
#[derive(Convert, Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Gender {
    #[default]
    Female,
    Genderless,
    Male,
}
