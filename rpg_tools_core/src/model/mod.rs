use crate::model::character::manager::CharacterMgr;
use crate::model::culture::{Culture, CultureId};
use crate::model::race::{Race, RaceId};
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
    pub character_manager: CharacterMgr,
    pub culture_manager: Storage<CultureId, Culture>,
    pub race_manager: Storage<RaceId, Race>,
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
            culture_manager: Default::default(),
            race_manager: Storage::new(Vec::new()),
        }
    }
}

pub fn get_setting_path(setting: &str, file: &str) -> PathBuf {
    ["resources", "settings", setting, file].iter().collect()
}
