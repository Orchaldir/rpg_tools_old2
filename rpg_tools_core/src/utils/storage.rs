pub trait Entry {
    fn new(id: usize) -> Self;

    fn id(&self) -> usize;
}

#[derive(Default, Debug)]
pub struct Storage<T: Entry> {
    entries: Vec<T>,
}

impl<T: Entry> Storage<T> {
    pub fn new(entries: Vec<T>) -> Self {
        Self { entries }
    }

    pub fn create(&mut self) -> usize {
        let id = self.entries.len();
        self.entries.push(T::new(id));
        id
    }

    pub fn get_all(&self) -> &Vec<T> {
        &self.entries
    }

    pub fn get(&self, id: usize) -> Option<&T> {
        self.entries.get(id)
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut T> {
        self.entries.get_mut(id)
    }
}
