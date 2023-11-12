use crate::model::race::gender::GenderOption;
use crate::model::race::RaceId;
use crate::model::RpgData;
use crate::utils::storage::Entry;
use anyhow::{bail, Context, Result};

/// Tries to update the name of a [`race`](crate::model::race::Race).
pub fn update_race_name(data: &mut RpgData, id: RaceId, name: &str) -> Result<()> {
    let trimmed = name.trim().to_string();

    if trimmed.is_empty() {
        bail!("Name is empty!")
    } else if data
        .race_manager
        .get_all()
        .iter()
        .filter(|r| r.id().ne(&id))
        .any(|r| r.name().eq(&trimmed))
    {
        bail!("Name '{}' already exists!", trimmed)
    }

    data.race_manager
        .get_mut(id)
        .map(|r| r.set_name(trimmed))
        .context("Race doesn't exist")?;

    Ok(())
}

/// Tries to update the gender option of a [`race`](crate::model::race::Race).
pub fn update_gender_option(data: &mut RpgData, id: RaceId, option: GenderOption) -> Result<()> {
    if data
        .race_manager
        .get(id)
        .map(|r| r.gender_option() == option)
        .context("Race doesn't exist")?
    {
        return Ok(());
    }

    if !data.character_manager.get_all().is_empty() {
        bail!("Cannot change, because the race is used by characters!")
    }

    if let Some(r) = data.race_manager.get_mut(id) {
        r.set_gender_option(option)
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use GenderOption::{NoGender, TwoGenders};

    #[test]
    fn test_empty_name() {
        let mut data = RpgData::default();

        assert!(update_race_name(&mut data, RaceId::default(), "").is_err());
    }

    #[test]
    fn test_name_contains_only_whitespaces() {
        let mut data = RpgData::default();

        assert!(update_race_name(&mut data, RaceId::default(), "  ").is_err());
    }

    #[test]
    fn test_update_name_of_non_existing_race() {
        let mut data = RpgData::default();

        assert!(update_race_name(&mut data, RaceId::default(), "Test").is_err());
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
        let id = data.race_manager.create();

        assert!(update_race_name(&mut data, id, input).is_ok());

        assert_eq!(result, data.race_manager.get(id).map(|r| r.name()).unwrap());
    }

    #[test]
    fn test_update_duplicate_name() {
        let mut data = RpgData::default();
        let id0 = data.race_manager.create();
        let id1 = data.race_manager.create();

        assert!(update_race_name(&mut data, id0, "Test").is_ok());
        assert!(update_race_name(&mut data, id1, "Test").is_err());
    }

    #[test]
    fn test_update_gender_of_non_existing_race() {
        let mut data = RpgData::default();

        assert!(update_gender_option(&mut data, RaceId::default(), TwoGenders).is_err());
    }

    #[test]
    fn test_update_gender_options() {
        test_update_gender_option(NoGender);
        test_update_gender_option(TwoGenders);
    }

    fn test_update_gender_option(option: GenderOption) {
        let mut data = RpgData::default();
        let id = data.race_manager.create();

        assert!(update_gender_option(&mut data, id, option).is_ok());

        assert_eq!(
            option,
            data.race_manager
                .get(id)
                .map(|r| r.gender_option())
                .unwrap()
        );
    }

    #[test]
    fn test_update_gender_options_while_used_by_characters() {
        let mut data = RpgData::default();
        let id = data.race_manager.create();
        data.character_manager.create();

        assert!(update_gender_option(&mut data, id, NoGender).is_err());
    }

    #[test]
    fn test_update_gender_options_ignore_characters_if_unchanged() {
        let mut data = RpgData::default();
        let id = data.race_manager.create();
        data.character_manager.create();

        assert!(update_gender_option(&mut data, id, TwoGenders).is_ok());
    }
}
