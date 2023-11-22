use crate::model::character::relation::relationship::Relationship;
use crate::model::character::{Character, CharacterId};
use crate::model::culture::{Culture, CultureId};
use crate::model::race::{Race, RaceId};
use crate::utils::relation::RelationStorage;
use crate::utils::storage::Storage;
use std::path::PathBuf;

pub mod appearance;
pub mod character;
pub mod culture;
pub mod equipment;
pub mod race;

#[derive(Debug)]
pub struct RpgData {
    pub setting: String,
    pub character_manager: Storage<CharacterId, Character>,
    pub culture_manager: Storage<CultureId, Culture>,
    pub race_manager: Storage<RaceId, Race>,
    pub relations: Relations,
}

impl RpgData {
    pub fn empty(setting: String) -> Self {
        RpgData {
            setting,
            ..RpgData::default()
        }
    }

    pub fn get_path(&self, file: &str) -> PathBuf {
        get_setting_path(&self.setting, file)
    }
}

impl Default for RpgData {
    fn default() -> Self {
        Self {
            setting: "".to_string(),
            character_manager: Default::default(),
            culture_manager: Storage::default(),
            race_manager: Storage::default(),
            relations: Default::default(),
        }
    }
}

pub fn get_setting_path(setting: &str, file: &str) -> PathBuf {
    ["resources", "settings", setting, file].iter().collect()
}

#[derive(Debug, Default)]
pub struct Relations {
    pub relationships: RelationStorage<CharacterId, Relationship>,
}
