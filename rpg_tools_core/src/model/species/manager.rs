use crate::model::species::{Species, SpeciesId};

/// Manages & stores the [`species`](Species).
#[derive(Default, Debug)]
pub struct SpeciesMgr {
    species: Vec<Species>,
}

impl SpeciesMgr {
    pub fn new(species: Vec<Species>) -> Self {
        Self { species }
    }

    pub fn create(&mut self) -> SpeciesId {
        let id = SpeciesId::new(self.species.len());
        self.species.push(Species::new(id));
        id
    }

    pub fn get_all(&self) -> &Vec<Species> {
        &self.species
    }

    pub fn get(&self, id: SpeciesId) -> Option<&Species> {
        self.species.get(id.0)
    }

    pub fn get_mut(&mut self, id: SpeciesId) -> Option<&mut Species> {
        self.species.get_mut(id.0)
    }
}
