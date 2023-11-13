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
        return DeleteResult::Blocked {
            characters: blocking_characters,
        };
    }

    match data.race_manager.delete(id) {
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

        assert_eq!(NotFound, delete_race(&mut data, RaceId::default()));
    }

    #[test]
    fn test_success() {
        let mut data = RpgData::default();
        let id = data.race_manager.create();

        assert_eq!(Ok, delete_race(&mut data, id));
    }

    #[test]
    fn test_blocked_by_character() {
        let mut data = RpgData::default();
        let race_id = data.race_manager.create();
        let character_id = data.character_manager.create();
        data.character_manager
            .get_mut(character_id)
            .map(|character| character.set_race(race_id));

        assert_eq!(
            Blocked {
                characters: vec![character_id]
            },
            delete_race(&mut data, race_id)
        );
    }
}
