use crate::model::character::CharacterId;
use crate::model::RpgData;
use crate::usecase::delete::{BlockingReason, DeleteResult};
use crate::utils::storage::DeleteElementResult;

/// Tries to delete a [`character`](crate::model::character::Character).
pub fn delete_character(data: &mut RpgData, id: CharacterId) -> DeleteResult {
    let relations = data
        .relations
        .relationships
        .get_all_of(id)
        .map(|map| map.len())
        .unwrap_or_default();

    if relations > 0 {
        return DeleteResult::Blocked(BlockingReason {
            characters: vec![],
            relations,
        });
    }

    match data.character_manager.delete(id) {
        DeleteElementResult::NotFound => DeleteResult::NotFound,
        _ => DeleteResult::Ok,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use DeleteResult::*;

    #[test]
    fn test_not_found() {
        let mut data = RpgData::default();

        assert_eq!(
            NotFound,
            delete_character(&mut data, CharacterId::default())
        );
    }

    #[test]
    fn test_success() {
        let mut data = RpgData::default();
        let id = data.character_manager.create();

        assert_eq!(Ok, delete_character(&mut data, id));
    }
}
