use crate::model::culture::{Culture, CultureId};

/// Manages & stores the [`cultures`](Culture).
#[derive(Default, Debug)]
pub struct CultureMgr {
    cultures: Vec<Culture>,
}

impl CultureMgr {
    pub fn new(cultures: Vec<Culture>) -> Self {
        Self { cultures }
    }

    pub fn create(&mut self) -> CultureId {
        let id = CultureId::new(self.cultures.len());
        self.cultures.push(Culture::new(id));
        id
    }

    pub fn get_all(&self) -> &Vec<Culture> {
        &self.cultures
    }

    pub fn get(&self, id: CultureId) -> Option<&Culture> {
        self.cultures.get(id.0)
    }

    pub fn get_mut(&mut self, id: CultureId) -> Option<&mut Culture> {
        self.cultures.get_mut(id.0)
    }
}
