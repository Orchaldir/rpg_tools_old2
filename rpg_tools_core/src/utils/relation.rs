use crate::utils::storage::Id;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Debug)]
pub struct RelationStorage<I: Id, T: Clone> {
    relations: HashMap<I, HashMap<I, T>>,
}

impl<I: Id, T: Clone> RelationStorage<I, T> {
    pub fn new(relations: HashMap<I, HashMap<I, T>>) -> Self {
        Self { relations }
    }

    /// Does it contain a specific element?.
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
}

impl<I: Id, T: Clone> Default for RelationStorage<I, T> {
    fn default() -> Self {
        RelationStorage::new(HashMap::new())
    }
}
