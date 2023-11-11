use crate::model::race::{Race, RaceId};

/// Manages & stores the [`races`](Race).
#[derive(Default, Debug)]
pub struct RaceMgr {
    races: Vec<Race>,
}

impl RaceMgr {
    pub fn new(races: Vec<Race>) -> Self {
        Self { races }
    }

    pub fn create(&mut self) -> RaceId {
        let id = RaceId::new(self.races.len());
        self.races.push(Race::new(id));
        id
    }

    pub fn get_all(&self) -> &Vec<Race> {
        &self.races
    }

    pub fn get(&self, id: RaceId) -> Option<&Race> {
        self.races.get(id.0)
    }

    pub fn get_mut(&mut self, id: RaceId) -> Option<&mut Race> {
        self.races.get_mut(id.0)
    }
}
