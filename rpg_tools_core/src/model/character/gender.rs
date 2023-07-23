use serde::Serialize;
use std::fmt;

/// The gender of the [`character`](crate::model::character::Character).
#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize)]
pub enum Gender {
    #[default]
    Female,
    Genderless,
    Male,
}

impl Gender {
    pub fn get_all() -> Vec<Gender> {
        vec![Gender::Female, Gender::Genderless, Gender::Male]
    }
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<&str> for Gender {
    fn from(gender: &str) -> Self {
        match gender {
            "Female" => Gender::Female,
            "Male" => Gender::Male,
            _ => Gender::Genderless,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        for gender in Gender::get_all() {
            let string = gender.to_string();
            assert_eq!(gender, Gender::from(&*string));
        }
    }
}
