use crate::model::character::gender::Gender;
use serde::{Deserialize, Serialize};

/// Which [`genders`](Gender) are available for members of this [`species`](crate::model::species::Species)?
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum GenderOption {
    NoGender,
    TwoGenders,
}

impl GenderOption {
    /// Is the [`gender`](Gender) valid for this option?
    ///
    /// ```
    ///# use rpg_tools_core::model::species::gender::GenderOption::*;
    ///# use rpg_tools_core::model::character::gender::Gender::*;
    ///
    /// assert!(!NoGender.is_valid(Female));
    /// assert!(!NoGender.is_valid(Male));
    /// assert!(NoGender.is_valid(Genderless));
    ///
    /// assert!(TwoGenders.is_valid(Female));
    /// assert!(TwoGenders.is_valid(Male));
    /// assert!(!TwoGenders.is_valid(Genderless));
    /// ```
    pub fn is_valid(&self, gender: Gender) -> bool {
        match self {
            GenderOption::NoGender => gender == Gender::Genderless,
            GenderOption::TwoGenders => gender == Gender::Female || gender == Gender::Male,
        }
    }
}
