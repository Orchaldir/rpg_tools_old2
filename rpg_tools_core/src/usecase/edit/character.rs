use crate::model::character::gender::Gender;
use crate::model::character::CharacterId;
use crate::model::RpgData;
use anyhow::{bail, Context, Result};

/// Tries to update the name of a [`character`](crate::model::character::Character).
pub fn update_character_name(data: &mut RpgData, id: CharacterId, name: &str) -> Result<()> {
    let trimmed = name.trim().to_string();

    if trimmed.is_empty() {
        bail!("Name is empty!")
    } else if data
        .character_manager
        .get_all()
        .iter()
        .filter(|r| r.id().ne(&id))
        .any(|r| r.name().eq(&trimmed))
    {
        bail!("Name '{}' already exists!", trimmed)
    }

    data.character_manager
        .get_mut(id)
        .map(|r| r.set_name(trimmed))
        .context("Character doesn't exist!")?;

    Ok(())
}

/// Tries to update the [`gender`](Gender) of a [`character`](crate::model::character::Character).
pub fn update_character_gender(data: &mut RpgData, id: CharacterId, gender: Gender) -> Result<()> {
    let race_id = data
        .character_manager
        .get(id)
        .map(|c| c.race())
        .context("Character doesn't exist!")?;
    let option = data
        .race_manager
        .get(race_id)
        .map(|r| r.gender_option())
        .context("Character's race doesn't exist!")?;

    if !option.is_valid(gender) {
        bail!("Gender is not valid for the race's gender option!");
    }

    if let Some(c) = data.character_manager.get_mut(id) {
        c.set_gender(gender)
    }

    Ok(())
}

/// Tries to update the [`race`](crate::model::race::Race) of a [`character`](crate::model::character::Character).
pub fn update_character_race(data: &mut RpgData, id: CharacterId, race_name: &str) -> Result<()> {
    let race = data
        .race_manager
        .get_all()
        .iter()
        .find(|race| race.name().eq(race_name))
        .context("Race doesn't exist!")?;
    let gender = data
        .character_manager
        .get(id)
        .map(|c| c.gender())
        .context("Character doesn't exist!")?;

    if !race.gender_option().is_valid(gender) {
        bail!("Race's gender option conflicts with the gender!")
    }

    let race_id = *race.id();

    if let Some(r) = data.character_manager.get_mut(id) {
        r.set_race(race_id)
    }

    Ok(())
}

/// Tries to update the [`culture`](crate::model::culture::Culture) of a [`character`](crate::model::character::Character).
pub fn update_character_culture(
    data: &mut RpgData,
    id: CharacterId,
    culture_name: &str,
) -> Result<()> {
    let culture_id = data
        .culture_manager
        .get_all()
        .iter()
        .find(|culture| culture.name().eq(culture_name))
        .map(|culture| *culture.id())
        .context("Culture doesn't exist!")?;

    data.character_manager
        .get_mut(id)
        .map(|character| character.set_culture(culture_id))
        .context("Character doesn't exist!")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::character::gender::Gender::Female;
    use Gender::{Genderless, Male};

    // update_character_name()

    #[test]
    fn test_empty_name() {
        let mut data = RpgData::default();

        assert!(update_character_name(&mut data, CharacterId::new(0), "").is_err());
    }

    #[test]
    fn test_name_contains_only_whitespaces() {
        let mut data = RpgData::default();

        assert!(update_character_name(&mut data, CharacterId::new(0), "  ").is_err());
    }

    #[test]
    fn test_update_name_of_non_existing_race() {
        let mut data = RpgData::default();

        assert!(update_character_name(&mut data, CharacterId::new(0), "Test").is_err());
    }

    #[test]
    fn test_update_valid_name() {
        test_update_name("Test", "Test");
    }

    #[test]
    fn test_update_trimmed_name() {
        test_update_name(" Name ", "Name");
    }

    fn test_update_name(input: &str, result: &str) {
        let mut data = RpgData::default();
        let id = data.character_manager.create();

        assert!(update_character_name(&mut data, id, input).is_ok());

        assert_eq!(
            result,
            data.character_manager.get(id).map(|r| r.name()).unwrap()
        );
    }

    #[test]
    fn test_update_duplicate_name() {
        let mut data = RpgData::default();
        let id0 = data.character_manager.create();
        let id1 = data.character_manager.create();

        assert!(update_character_name(&mut data, id0, "Test").is_ok());
        assert!(update_character_name(&mut data, id1, "Test").is_err());
    }

    // update_character_gender()

    #[test]
    fn test_update_gender_of_non_existing_character() {
        let mut data = RpgData::default();

        assert!(update_character_gender(&mut data, CharacterId::new(0), Male).is_err());
    }

    #[test]
    fn test_update_gender_with_non_existing_race() {
        let mut data = RpgData::default();
        let character_id = data.character_manager.create();

        assert!(update_character_gender(&mut data, character_id, Male).is_err());
    }

    #[test]
    fn test_update_genders() {
        test_update_gender(Male);
        test_update_gender(Female);
    }

    fn test_update_gender(gender: Gender) {
        let mut data = RpgData::default();
        data.race_manager.create();
        let character_id = data.character_manager.create();

        assert!(update_character_gender(&mut data, character_id, gender).is_ok());

        assert_eq!(
            gender,
            data.character_manager
                .get(character_id)
                .map(|r| r.gender())
                .unwrap()
        );
    }

    #[test]
    fn test_update_invalid_genders() {
        let mut data = RpgData::default();
        data.race_manager.create();
        let character_id = data.character_manager.create();

        assert!(update_character_gender(&mut data, character_id, Genderless).is_err());
    }

    // update_character_race()

    #[test]
    fn test_update_race_with_non_existing_race() {
        let mut data = RpgData::default();
        let character_id = data.character_manager.create();

        assert!(update_character_race(&mut data, character_id, "Test").is_err());
    }

    #[test]
    fn test_update_race_of_non_existing_character() {
        let mut data = RpgData::default();
        let race_id = data.race_manager.create();
        data.race_manager
            .get_mut(race_id)
            .map(|r| r.set_name("Test".to_string()));

        assert!(update_character_race(&mut data, CharacterId::new(0), "Test").is_err());
    }

    #[test]
    fn test_update_race() {
        let mut data = RpgData::default();
        let character_id = data.character_manager.create();
        let race_id = data.race_manager.create();
        data.race_manager
            .get_mut(race_id)
            .map(|r| r.set_name("Test".to_string()));

        assert!(update_character_race(&mut data, character_id, "Test").is_ok());

        assert_eq!(
            race_id,
            data.character_manager
                .get(character_id)
                .map(|r| r.race())
                .unwrap()
        );
    }

    #[test]
    fn test_update_race_with_invalid_gender() {
        let mut data = RpgData::default();
        let character_id = data.character_manager.create();
        data.character_manager
            .get_mut(character_id)
            .map(|c| c.set_gender(Genderless));
        let race_id = data.race_manager.create();
        data.race_manager
            .get_mut(race_id)
            .map(|r| r.set_name("Test".to_string()));

        assert!(update_character_race(&mut data, character_id, "Test").is_err());
    }

    // update_character_culture()

    #[test]
    fn test_update_culture_with_non_existing_culture() {
        let mut data = RpgData::default();
        let character_id = data.character_manager.create();

        assert!(update_character_culture(&mut data, character_id, "Test").is_err());
    }

    #[test]
    fn test_update_culture_of_non_existing_character() {
        let mut data = RpgData::default();
        let culture_id = data.culture_manager.create();
        data.culture_manager
            .get_mut(culture_id)
            .map(|r| r.set_name("Test".to_string()));

        assert!(update_character_culture(&mut data, CharacterId::new(0), "Test").is_err());
    }

    #[test]
    fn test_update_culture() {
        let mut data = RpgData::default();
        let character_id = data.character_manager.create();
        let culture_id = data.culture_manager.create();
        data.culture_manager
            .get_mut(culture_id)
            .map(|r| r.set_name("Test".to_string()));

        assert!(update_character_culture(&mut data, character_id, "Test").is_ok());

        assert_eq!(
            culture_id,
            data.character_manager
                .get(character_id)
                .map(|r| r.culture())
                .unwrap()
        );
    }
}
