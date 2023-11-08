use crate::EditorData;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use rpg_tools_core::model::species::gender::GenderOption;
use rpg_tools_core::model::species::{Species, SpeciesId};

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
                gender_option: format!("{:?}", species.gender_option()),
            },
        )
    })
}

#[get("/species/edit/<id>")]
pub fn edit_species(data: &State<EditorData>, id: usize) -> Option<Template> {
    let data = data.data.lock().expect("lock shared data");
    data.species_manager
        .get(SpeciesId::new(id))
        .map(|species| edit_species_template(id, species))
}

fn edit_species_template(id: usize, species: &Species) -> Template {
    Template::render(
        "species/edit",
        context! {
            id: id,
            name: species.name(),
            gender_options: GenderOption::get_all(),
            gender_option: species.gender_option(),
        },
    )
}
