use crate::model::race::gender::GenderOption;
use serde::{Deserialize, Serialize};

pub mod gender;
pub mod manager;

/// The unique identifier of a [`race`](Race).
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct RaceId(usize);

impl RaceId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn id(&self) -> usize {
        self.0
    }
}

/// A race like human, elf or dragon.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Race {
    id: RaceId,
    name: String,
    gender_option: GenderOption,
}

impl Race {
    pub fn new(id: RaceId) -> Self {
        Race {
            id,
            name: format!("Race {}", id.0),
            gender_option: GenderOption::TwoGenders,
        }
    }

    pub fn id(&self) -> &RaceId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn gender_option(&self) -> GenderOption {
        self.gender_option
    }

    pub fn set_gender_option(&mut self, gender_option: GenderOption) {
        self.gender_option = gender_option;
    }
}
