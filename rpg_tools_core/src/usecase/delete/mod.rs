use crate::model::character::CharacterId;

pub mod culture;
pub mod race;

pub enum DeleteResult {
    Ok,
    NotFound,
    Blocked { characters: Vec<CharacterId> },
}
