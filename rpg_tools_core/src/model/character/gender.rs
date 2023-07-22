use serde::Serialize;

/// The gender of the [`character`](crate::model::character::Character).
#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize)]
pub enum Gender {
    #[default]
    Female,
    Genderless,
    Male,
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
