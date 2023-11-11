use crate::model::character::manager::CharacterMgr;
use crate::model::culture::manager::CultureMgr;
use crate::model::race::manager::RaceMgr;

pub mod character;
pub mod color;
pub mod culture;
pub mod equipment;
pub mod length;
pub mod race;
pub mod side;
pub mod size;
pub mod transparency;
pub mod width;

#[derive(Default, Debug)]
pub struct RpgData {
    pub character_manager: CharacterMgr,
    pub culture_manager: CultureMgr,
    pub race_manager: RaceMgr,
}
