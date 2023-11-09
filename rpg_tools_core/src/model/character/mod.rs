use crate::model::character::appearance::Appearance;
use crate::model::character::gender::Gender;
use serde::{Deserialize, Serialize};
use crate::model::race::RaceId;

pub mod appearance;
pub mod gender;
pub mod manager;

/// The unique identifier of a [`character`](Character).
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Character {
    id: CharacterId,
    name: String,
    race: RaceId,
    gender: Gender,
    appearance: Appearance,
}

impl Character {
    pub fn new(id: CharacterId) -> Self {
        Character {
            id,
            name: format!("Character {}", id.0),
            race: RaceId::new(0),
            gender: Gender::default(),
            appearance: Appearance::default(),
        }
    }

    pub fn id(&self) -> &CharacterId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn race(&self) -> RaceId {
        self.race
    }

    pub fn set_race(&mut self, race: RaceId) {
        self.race = race;
    }

    pub fn gender(&self) -> Gender {
        self.gender
    }

    pub fn set_gender(&mut self, gender: Gender) {
        self.gender = gender;
    }

    pub fn appearance(&self) -> &Appearance {
        &self.appearance
    }

    pub fn set_appearance(&mut self, appearance: Appearance) {
        self.appearance = appearance;
    }
}
