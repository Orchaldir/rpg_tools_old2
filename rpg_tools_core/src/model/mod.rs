use crate::model::character::manager::CharacterMgr;
use crate::model::species::manager::SpeciesMgr;

pub mod character;
pub mod color;
pub mod equipment;
pub mod length;
pub mod side;
pub mod size;
pub mod species;
pub mod transparency;
pub mod width;

#[derive(Default, Debug)]
pub struct RpgData {
    pub species_manager: SpeciesMgr,
    pub character_manager: CharacterMgr,
}
