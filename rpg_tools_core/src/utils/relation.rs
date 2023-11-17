use crate::utils::storage::Id;
use std::collections::HashMap;

#[derive(Debug)]
pub struct RelationStorage<I: Id, T> {
    relations: HashMap<I, HashMap<I, T>>,
}

impl<I: Id, T> RelationStorage<I, T> {
    pub fn new(relations: HashMap<I, HashMap<I, T>>) -> Self {
        Self { relations }
    }

    /// Gets all relations for a specific element.
    pub fn get(&self, id: I) -> Option<&HashMap<I, T>> {
        self.relations.get(&id)
    }

    /// Gets the relation between 2 elements.
    pub fn get_between(&self, from: I, to: I) -> Option<&T> {
        self.relations.get(&from).and_then(|map| map.get(&to))
    }

    /// Adds a relation between 2 elements.
    pub fn add_between(&mut self, from: I, to: I, relation: T) {
        self.relations.entry(from)
            .or_default()
            .insert(to, relation);
    }
}

impl<I: Id, T> Default for RelationStorage<I, T> {
    fn default() -> Self {
        RelationStorage::new(HashMap::new())
    }
}
