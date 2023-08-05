use crate::model::character::{Character, CharacterId};

/// Manages & stores the [`characters`](Character).
#[derive(Default, Debug)]
pub struct CharacterMgr {
    characters: Vec<Character>,
}

impl CharacterMgr {
    pub fn new(characters: Vec<Character>) -> Self {
        Self { characters }
    }

    pub fn create(&mut self) -> CharacterId {
        let id = CharacterId::new(self.characters.len());
        self.characters.push(Character::new(id));
        id
    }

    pub fn get_all(&self) -> &Vec<Character> {
        &self.characters
    }

    pub fn get(&self, id: CharacterId) -> Option<&Character> {
        self.characters.get(id.0)
    }

    pub fn get_mut(&mut self, id: CharacterId) -> Option<&mut Character> {
        self.characters.get_mut(id.0)
    }
}
