use crate::model::race::RaceId;
use crate::model::RpgData;
use anyhow::{bail, Context, Result};

/// Tries to update the name of a race.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_name() {
        let mut data = RpgData::default();

        assert!(update_race_name(&mut data, RaceId::new(0), "").is_err());
    }

    #[test]
    fn test_name_contains_only_whitespaces() {
        let mut data = RpgData::default();

        assert!(update_race_name(&mut data, RaceId::new(0), "  ").is_err());
    }

    #[test]
    fn test_update_name() {
        let mut data = RpgData::default();
        let id = data.race_manager.create();

        assert!(update_race_name(&mut data, id, "Test").is_ok());

        assert_eq!("Test", data.race_manager.get(id).map(|r| r.name()).unwrap());
    }
}
