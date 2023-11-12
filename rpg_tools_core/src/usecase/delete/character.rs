use crate::model::character::CharacterId;
use crate::model::RpgData;
use crate::usecase::delete::DeleteResult;

/// Tries to delete a [`character`](crate::model::character::Character).
pub fn delete_character(data: &mut RpgData, id: CharacterId) -> DeleteResult {
    match data.character_manager.delete(id) {
        None => DeleteResult::NotFound,
        Some(_) => DeleteResult::Ok,
    }
}
