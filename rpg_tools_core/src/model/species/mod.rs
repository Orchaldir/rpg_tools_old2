use crate::model::species::gender::GenderOption;
use serde::{Deserialize, Serialize};

pub mod gender;
pub mod manager;

/// The unique identifier of a [`species`](Species).
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SpeciesId(usize);

impl SpeciesId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn id(&self) -> usize {
        self.0
    }
}

/// A species like human, elf or dragon.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Species {
    id: SpeciesId,
    name: String,
    gender_option: GenderOption,
}

impl Species {
    pub fn new(id: SpeciesId) -> Self {
        Species {
            id,
            name: format!("Species {}", id.0),
            gender_option: GenderOption::TwoGenders,
        }
    }

    pub fn id(&self) -> &SpeciesId {
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
