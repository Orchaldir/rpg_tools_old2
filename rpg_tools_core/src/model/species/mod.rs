use serde::{Deserialize, Serialize};

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

/// A species in the game.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Species {
    id: SpeciesId,
    name: String,
}

impl Species {
    pub fn new(id: SpeciesId) -> Self {
        Species {
            id,
            name: format!("Species {}", id.0),
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
}
