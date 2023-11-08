use crate::EditorData;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use rpg_tools_core::model::species::SpeciesId;

#[get("/species/all")]
pub fn get_all_species(data: &State<EditorData>) -> Template {
    let data = data.data.lock().expect("lock shared data");
    let species: Vec<(usize, &str)> = data
        .species_manager
        .get_all()
        .iter()
        .map(|c| (c.id().id(), c.name()))
        .collect();

    Template::render(
        "species/all",
        context! {
            total: species.len(),
            species: species,
        },
    )
}

#[get("/species/details/<id>")]
pub fn get_species_details(data: &State<EditorData>, id: usize) -> Option<Template> {
    let data = data.data.lock().expect("lock shared data");

    data.species_manager.get(SpeciesId::new(id)).map(|species| {
        Template::render(
            "species/details",
            context! {
                name: species.name(),
                id: id,
                gender: format!("{:?}", species.gender_option()),
            },
        )
    })
}
