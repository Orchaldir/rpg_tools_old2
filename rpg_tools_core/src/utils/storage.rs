use std::marker::PhantomData;

pub trait Id: Copy {
    fn new(id: usize) -> Self;

    fn id(&self) -> usize;
}

pub trait Entry<I: Id> {
    fn new(id: I) -> Self;

    fn id(&self) -> I;
}

#[derive(Default, Debug)]
pub struct Storage<I: Id, T: Entry<I>> {
    entries: Vec<T>,
    phantom: PhantomData<I>,
}

impl<I: Id, T: Entry<I>> Storage<I, T> {
    pub fn new(entries: Vec<T>) -> Self {
        Self {
            entries,
            phantom: PhantomData,
        }
    }

    pub fn create(&mut self) -> I {
        let id = Id::new(self.entries.len());
        self.entries.push(T::new(id));
        id
    }

    pub fn get_all(&self) -> &Vec<T> {
        &self.entries
    }

    pub fn get(&self, id: I) -> Option<&T> {
        self.entries.get(id.id())
    }

    pub fn get_mut(&mut self, id: I) -> Option<&mut T> {
        self.entries.get_mut(id.id())
    }
}
