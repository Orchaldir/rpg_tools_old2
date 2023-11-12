use crate::utils::storage::{Entry, Id};
use serde::{Deserialize, Serialize};

/// The unique identifier of a [`culture`](Culture).
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CultureId(usize);

impl Id for CultureId {
    fn new(id: usize) -> Self {
        Self(id)
    }

    fn id(&self) -> usize {
        self.0
    }
}

/// A [`character's`](crate::model::character::Character) culture influences names, fashion and much more.
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Culture {
    id: CultureId,
    name: String,
}

impl Culture {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Entry<CultureId> for Culture {
    fn new(id: CultureId) -> Self {
        Culture {
            id,
            name: format!("Culture {}", id.0),
        }
    }

    fn id(&self) -> CultureId {
        self.id
    }
}
