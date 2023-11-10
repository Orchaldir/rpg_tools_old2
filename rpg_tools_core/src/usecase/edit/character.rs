use crate::model::character::CharacterId;
use crate::model::RpgData;
use anyhow::{bail, Context, Result};

/// Tries to update the name of a [`character`](crate::model::character::Character)..
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
        .context("Character doesn't exist")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
