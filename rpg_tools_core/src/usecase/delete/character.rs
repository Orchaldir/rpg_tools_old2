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
