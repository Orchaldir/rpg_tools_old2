use crate::model::character::CharacterId;

pub mod character;
pub mod culture;
pub mod race;

#[derive(Debug, PartialEq, Eq)]
pub enum DeleteResult {
    Ok,
    NotFound,
    Blocked { characters: Vec<CharacterId> },
}
