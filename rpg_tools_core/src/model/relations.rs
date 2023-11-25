use crate::model::character::relation::relationship::{Relationship, RELATIONSHIPS_FILE};
use crate::model::character::relation::romantic::{RomanticRelationship, ROMANTIC_FILE};
use crate::model::character::CharacterId;
use crate::model::get_setting_path;
use crate::utils::relation::{load_relations, RelationStorage};
use anyhow::{Context, Result};

#[derive(Debug, Default)]
pub struct Relations {
    pub relationships: RelationStorage<CharacterId, Relationship>,
    pub romantic: RelationStorage<CharacterId, RomanticRelationship>,
}

impl Relations {
    /// Loads all relations from a file.
    pub fn load(setting: &str) -> Result<Self> {
        let relationships = load_relations(&get_setting_path(setting, RELATIONSHIPS_FILE))?;
        let romantic = load_relations(&get_setting_path(setting, ROMANTIC_FILE))?;

        Ok(Self {
            relationships,
            romantic,
        })
    }

    /// Counts all relations of a character.
    pub fn count_for_character(&self, id: CharacterId) -> usize {
        self.relationships.count_all_of(id) + self.romantic.count_all_of(id)
    }

    /// Swaps a character id with another.
    pub fn swap_character(&mut self, old: CharacterId, new: CharacterId) -> Result<()> {
        self.relationships
            .swap(old, new)
            .context("Failed to swap character for relationships")?;
        self.romantic
            .swap(old, new)
            .context("Failed to swap character for romantic relationships")?;

        Ok(())
    }
}
