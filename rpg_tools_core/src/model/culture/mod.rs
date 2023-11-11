use serde::{Deserialize, Serialize};

pub mod manager;

/// The unique identifier of a [`culture`](Culture).
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CultureId(usize);

impl CultureId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn id(&self) -> usize {
        self.0
    }
}

/// A [`character's`](crate::model::character::Character) culture influences names, fashion and much more.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Culture {
    id: CultureId,
    name: String,
}

impl Culture {
    pub fn new(id: CultureId) -> Self {
        Culture {
            id,
            name: format!("Culture {}", id.0),
        }
    }

    pub fn id(&self) -> &CultureId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}
