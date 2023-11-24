use crate::model::character::relation::relationship::{Relationship, RELATIONSHIPS_FILE};
use crate::model::character::relation::romantic::{RomanticRelationship, ROMANTIC_FILE};
use crate::model::character::CharacterId;
use crate::model::get_setting_path;
use crate::utils::relation::RelationStorage;
use anyhow::Result;

#[derive(Debug, Default)]
pub struct Relations {
    pub relationships: RelationStorage<CharacterId, Relationship>,
    pub romantic: RelationStorage<CharacterId, RomanticRelationship>,
}

impl Relations {
    pub fn load(setting: &str) -> Result<Self> {
        let relationships = RelationStorage::load(&get_setting_path(setting, RELATIONSHIPS_FILE))?;
        let romantic = RelationStorage::load(&get_setting_path(setting, ROMANTIC_FILE))?;

        Ok(Self {
            relationships,
            romantic,
        })
    }
}
