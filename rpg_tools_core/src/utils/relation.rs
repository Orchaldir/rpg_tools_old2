use crate::utils::storage::Id;
use anyhow::{Context, Result};
use itertools::Itertools;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug)]
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

    /// Saves the relation to a file.
    pub fn save(&self, path: &Path) -> Result<()> {
        let mut file = File::create(path).context(format!("Failed to create {:?}", path))?;

        writeln!(file, "Id0,Id1,Relation")?;

        for (id0, relations0) in self.relations.iter().sorted_by_key(|x| x.0.id()) {
            for (id1, relations0) in relations0.iter().sorted_by_key(|x| x.0.id()) {
                if id0.id() < id1.id() {
                    writeln!(file, "{},{},{}", id0.id(), id1.id(), relations0)?;
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
