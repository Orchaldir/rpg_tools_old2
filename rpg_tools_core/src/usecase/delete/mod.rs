use crate::model::character::CharacterId;

pub mod character;
pub mod culture;
pub mod race;

#[derive(Debug, PartialEq, Eq)]
pub enum DeleteResult {
    Ok,
    NotFound,
    Blocked(BlockingReason),
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct BlockingReason {
    pub characters: Vec<CharacterId>,
    pub relations: usize,
}
