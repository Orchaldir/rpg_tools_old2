use crate::model::character::manager::CharacterMgr;
use crate::model::culture::{Culture, CultureId};
use crate::model::race::manager::RaceMgr;
use crate::utils::storage::Storage;
use std::path::PathBuf;

pub mod appearance;
pub mod character;
pub mod culture;
pub mod equipment;
pub mod race;

#[derive(Default, Debug)]
pub struct RpgData {
    pub setting: String,
    pub character_manager: CharacterMgr,
    pub culture_manager: Storage<CultureId, Culture>,
    pub race_manager: RaceMgr,
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

pub fn get_setting_path(setting: &str, file: &str) -> PathBuf {
    ["resources", "settings", setting, file].iter().collect()
}
