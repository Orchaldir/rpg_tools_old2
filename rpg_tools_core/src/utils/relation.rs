use crate::utils::storage::Id;
use anyhow::{bail, Context, Result};
use itertools::Itertools;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug, PartialEq)]
pub struct RelationStorage<I: Id, T: Clone> {
    relations: HashMap<I, HashMap<I, T>>,
}

impl<I: Id, T: Clone + Display> RelationStorage<I, T> {
    pub fn new(relations: HashMap<I, HashMap<I, T>>) -> Self {
        Self { relations }
    }

    /// Does it contain a specific element?
    pub fn contains(&self, id: I) -> bool {
        self.relations.contains_key(&id)
    }

    /// Counts all relations for a specific element.
    pub fn count_all_of(&self, id: I) -> usize {
        self.relations
            .get(&id)
            .map(|map| map.len())
            .unwrap_or_default()
    }

    /// Gets all relations for a specific element.
    pub fn get_all_of(&self, id: I) -> Option<&HashMap<I, T>> {
        self.relations.get(&id)
    }

    /// Gets the relation between 2 elements.
    pub fn get(&self, from: I, to: I) -> Option<&T> {
        self.relations.get(&from).and_then(|map| map.get(&to))
    }

    /// Adds a relation between 2 elements.
    pub fn add(&mut self, from: I, to: I, relation: T) {
        self.add_one_direction(from, to, relation.clone());
        self.add_one_direction(to, from, relation);
    }

    fn add_one_direction(&mut self, from: I, to: I, relation: T) {
        self.relations.entry(from).or_default().insert(to, relation);
    }

    /// Deletes the relation between 2 elements.
    pub fn delete(&mut self, from: I, to: I) {
        self.delete_one_direction(from, to);
        self.delete_one_direction(to, from);
    }

    fn delete_one_direction(&mut self, from: I, to: I) {
        if let Entry::Occupied(mut e) = self.relations.entry(from) {
            e.get_mut().remove(&to);
        }
    }

    /// Switches an id to another.
    pub fn update_id(&mut self, old: I, new: I) -> Result<()> {
        if self.contains(new) {
            bail!(
                "Cannot switch id from {} to {}, because it is already contained!",
                old.id(),
                new.id()
            )
        }

        if let Some(relations) = self.relations.remove(&old) {
            for (id1, relation) in relations {
                self.delete_one_direction(id1, old);
                self.add(new, id1, relation);
            }
        }

        Ok(())
    }

    /// Saves the relations to a file.
    pub fn save(&self, path: &Path) -> Result<()> {
        let mut file = File::create(path).context(format!("Failed to create {:?}", path))?;

        writeln!(file, "Id0,Id1,Relation")?;

        for (id0, relations) in self.relations.iter().sorted_by_key(|x| x.0.id()) {
            for (id1, relation) in relations.iter().sorted_by_key(|x| x.0.id()) {
                if id0.id() < id1.id() {
                    writeln!(file, "{},{},{}", id0.id(), id1.id(), relation)?;
                }
            }
        }

        Ok(())
    }
}

impl<I: Id, T: Clone + Display> Default for RelationStorage<I, T> {
    fn default() -> Self {
        RelationStorage::new(HashMap::new())
    }
}

/// Loads the relations from a file.
pub fn load_relations<I: Id, T: Clone + Display + for<'a> From<&'a str>>(
    path: &Path,
) -> Result<RelationStorage<I, T>> {
    let mut reader = csv::Reader::from_path(path).context(format!("Failed to load {:?}", path))?;
    let mut storage = RelationStorage::new(HashMap::new());

    for (line, result) in reader.records().enumerate() {
        let record = result.with_context(|| format!("Cannot read line {}", line))?;

        let id0: usize = record[0]
            .parse()
            .with_context(|| format!("Failed to parse id0 of line {}", line))?;
        let id1: usize = record[1]
            .parse()
            .with_context(|| format!("Failed to parse id1 of line {}", line))?;
        let relation: T = record[2].into();
        storage.add(Id::new(id0), Id::new(id1), relation);
    }

    Ok(storage)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::character::relation::relationship::Relationship;
    use crate::model::character::relation::relationship::Relationship::{Enemy, Friend};
    use crate::model::character::CharacterId;
    use tempdir::TempDir;

    #[test]
    fn test_io() {
        let storage = init_storage();

        let dir = TempDir::new("test").unwrap();
        let file_path = dir.path().join("relations.csv");

        assert!(storage.save(&file_path).is_ok());
        assert_eq!(storage, load_relations(&file_path).unwrap());
    }

    #[test]
    fn test_switch_unknown_new() {
        let mut storage = init_storage();

        assert!(storage
            .update_id(CharacterId::new(0), CharacterId::new(1))
            .is_ok());
        assert_eq!(
            storage.get(CharacterId::new(2), CharacterId::new(3)),
            Some(&Friend)
        );
        assert_eq!(
            storage.get(CharacterId::new(2), CharacterId::new(4)),
            Some(&Enemy)
        );
    }

    fn init_storage() -> RelationStorage<CharacterId, Relationship> {
        let mut storage = RelationStorage::default();
        let id0 = CharacterId::new(2);
        storage.add(id0, CharacterId::new(3), Friend);
        storage.add(id0, CharacterId::new(4), Enemy);
        storage
    }
}
