use crate::model::character::CharacterId;
use crate::model::race::RaceId;
use crate::model::RpgData;
use crate::usecase::delete::DeleteResult;
use crate::utils::storage::Entry;

/// Tries to delete a [`race`](crate::model::race::Race).
pub fn delete_race(data: &mut RpgData, id: RaceId) -> DeleteResult {
    let blocking_characters: Vec<CharacterId> = data
        .character_manager
        .get_all()
        .iter()
        .filter(|character| character.race().eq(&id))
        .map(|character| character.id())
        .collect();

    if !blocking_characters.is_empty() {
        return DeleteResult::Blocked;
    }

    match data.race_manager.delete(id) {
        None => DeleteResult::NotFound,
        Some(_) => DeleteResult::Ok,
    }
}
