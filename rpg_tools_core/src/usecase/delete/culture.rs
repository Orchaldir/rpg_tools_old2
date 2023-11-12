use crate::model::character::CharacterId;
use crate::model::culture::CultureId;
use crate::model::RpgData;
use crate::usecase::delete::DeleteResult;
use crate::utils::storage::Entry;

/// Tries to delete a [`culture`](crate::model::culture::Culture).
pub fn delete_culture(data: &mut RpgData, id: CultureId) -> DeleteResult {
    let blocking_characters: Vec<CharacterId> = data
        .character_manager
        .get_all()
        .iter()
        .filter(|character| character.culture().eq(&id))
        .map(|character| character.id())
        .collect();

    if !blocking_characters.is_empty() {
        return DeleteResult::Blocked {
            characters: blocking_characters,
        };
    }

    match data.culture_manager.delete(id) {
        None => DeleteResult::NotFound,
        Some(_) => DeleteResult::Ok,
    }
}
