use std::hash::Hash;
use std::marker::PhantomData;

pub trait Id: Copy + Hash + Eq {
    fn new(id: usize) -> Self;

    fn id(&self) -> usize;
}

pub trait Element<I: Id> {
    fn new(id: I) -> Self;

    fn id(&self) -> I;

    fn set_id(&mut self, id: I);
}

#[derive(Debug, PartialEq, Eq)]
pub enum DeleteElementResult<I: Id> {
    DeletedLastElement,
    SwappedAndRemoved { id_to_update: I },
    NotFound,
}

#[derive(Debug)]
pub struct Storage<I: Id, T: Element<I>> {
    elements: Vec<T>,
    phantom: PhantomData<I>,
}

impl<I: Id, T: Element<I>> Storage<I, T> {
    pub fn new(entries: Vec<T>) -> Self {
        Self {
            elements: entries,
            phantom: PhantomData,
        }
    }

    pub fn create(&mut self) -> I {
        let id = Id::new(self.elements.len());
        self.elements.push(T::new(id));
        id
    }

    pub fn get_all(&self) -> &Vec<T> {
        &self.elements
    }

    pub fn get_all_mut(&mut self) -> &mut Vec<T> {
        &mut self.elements
    }

    pub fn get(&self, id: I) -> Option<&T> {
        self.elements.get(id.id())
    }

    pub fn get_mut(&mut self, id: I) -> Option<&mut T> {
        self.elements.get_mut(id.id())
    }

    /// Deletes an element by swapping it with the last one, if necessary.
    pub fn delete(&mut self, id: I) -> DeleteElementResult<I> {
        let len = self.elements.len();

        if id.id() >= len {
            return DeleteElementResult::NotFound;
        } else if id.id() + 1 == len {
            self.elements.pop();
            return DeleteElementResult::DeletedLastElement;
        }

        self.elements.swap_remove(id.id());
        self.elements[id.id()].set_id(id);

        DeleteElementResult::SwappedAndRemoved {
            id_to_update: I::new(len - 1),
        }
    }
}

impl<I: Id, T: Element<I>> Default for Storage<I, T> {
    fn default() -> Self {
        Storage::new(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::character::{Character, CharacterId};
    use crate::utils::storage::DeleteElementResult::*;

    #[test]
    fn test_delete_element_in_empty_storage() {
        let mut storage: Storage<CharacterId, Character> = Storage::default();

        assert_eq!(NotFound, storage.delete(CharacterId::default()));
    }

    #[test]
    fn test_delete_only_element() {
        let mut storage: Storage<CharacterId, Character> = Storage::default();
        let id = storage.create();

        assert_eq!(DeletedLastElement, storage.delete(id));
        assert!(storage.get_all().is_empty());
    }

    #[test]
    fn test_delete() {
        let mut storage: Storage<CharacterId, Character> = Storage::default();
        let id0 = storage.create();
        let id1 = storage.create();
        let id2 = storage.create();

        assert_eq!(SwappedAndRemoved { id_to_update: id2 }, storage.delete(id0));

        assert_eq!(2, storage.get_all().len());
        assert_element(storage.get(id0).unwrap(), id0, "Character 2");
        assert_element(storage.get(id1).unwrap(), id1, "Character 1");
    }

    #[test]
    fn test_delete_unknown_index() {
        let mut storage: Storage<CharacterId, Character> = Storage::default();
        let id = storage.create();

        assert_eq!(NotFound, storage.delete(CharacterId::new(5)));
        assert_eq!(1, storage.get_all().len());
        assert_element(storage.get(id).unwrap(), id, "Character 0");
    }

    fn assert_element(element: &Character, id: CharacterId, name: &str) {
        assert_eq!(id, element.id());
        assert_eq!(name, element.name());
    }
}
