use crate::model::character::appearance::Appearance;

pub mod appearance;

/// The unique identifier of a [`character`](Character).
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct CharacterId(usize);

impl CharacterId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn id(&self) -> usize {
        self.0
    }
}

/// A character in the game.
#[derive(Clone, Debug, PartialEq)]
pub struct Character {
    id: CharacterId,
    name: String,
    appearance: Appearance,
}

impl Character {
    pub fn new(id: CharacterId) -> Self {
        Character {
            id,
            name: id.0.to_string(),
            appearance: Appearance::default(),
        }
    }
}
