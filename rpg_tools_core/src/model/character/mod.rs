use crate::model::character::appearance::Appearance;
use crate::model::character::gender::Gender;
use crate::model::culture::CultureId;
use crate::model::race::RaceId;
use crate::utils::storage::{Entry, Id};
use serde::{Deserialize, Serialize};

pub mod appearance;
pub mod gender;

/// The unique identifier of a [`character`](Character).
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CharacterId(usize);

impl Id for CharacterId {
    fn new(id: usize) -> Self {
        Self(id)
    }

    fn id(&self) -> usize {
        self.0
    }
}

/// A character in the game.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Character {
    id: CharacterId,
    name: String,
    race: RaceId,
    culture: CultureId,
    gender: Gender,
    appearance: Appearance,
}

impl Character {
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

    pub fn culture(&self) -> CultureId {
        self.culture
    }

    pub fn set_culture(&mut self, culture: CultureId) {
        self.culture = culture;
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

impl Entry<CharacterId> for Character {
    fn new(id: CharacterId) -> Self {
        Character {
            id,
            name: format!("Character {}", id.0),
            race: RaceId::default(),
            culture: CultureId::default(),
            gender: Gender::default(),
            appearance: Appearance::default(),
        }
    }

    fn id(&self) -> CharacterId {
        self.id
    }

    fn set_id(&mut self, id: CharacterId) {
        self.id = id;
    }
}
