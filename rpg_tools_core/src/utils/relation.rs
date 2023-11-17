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

    /// Gets all relations for the element with *id*.
    pub fn get(&self, id: I) -> Option<&HashMap<I, T>> {
        self.relations.get(&id)
    }
}

impl<I: Id, T> Default for RelationStorage<I, T> {
    fn default() -> Self {
        RelationStorage::new(HashMap::new())
    }
}
