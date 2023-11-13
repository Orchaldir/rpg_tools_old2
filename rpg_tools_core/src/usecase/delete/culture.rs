use crate::model::character::CharacterId;
use crate::model::culture::CultureId;
use crate::model::RpgData;
use crate::usecase::delete::DeleteResult;
use crate::utils::storage::{DeleteElementResult, Entry};

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

        assert_eq!(NotFound, delete_culture(&mut data, CultureId::default()));
    }

    #[test]
    fn test_success() {
        let mut data = RpgData::default();
        let id = data.culture_manager.create();

        assert_eq!(Ok, delete_culture(&mut data, id));
    }

    #[test]
    fn test_blocked_by_character() {
        let mut data = RpgData::default();
        let culture_id = data.culture_manager.create();
        let character_id = data.character_manager.create();
        data.character_manager
            .get_mut(character_id)
            .map(|character| character.set_culture(culture_id));

        assert_eq!(
            Blocked {
                characters: vec![character_id]
            },
            delete_culture(&mut data, culture_id)
        );
    }
}
