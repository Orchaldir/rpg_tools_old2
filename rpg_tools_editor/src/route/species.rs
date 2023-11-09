use crate::EditorData;
use rocket::form::Form;
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

    data.species_manager
        .get(SpeciesId::new(id))
        .map(|species| get_details_template(id, species))
}

#[get("/species/edit/<id>")]
pub fn edit_species(data: &State<EditorData>, id: usize) -> Option<Template> {
    let data = data.data.lock().expect("lock shared data");
    data.species_manager
        .get(SpeciesId::new(id))
        .map(|species| get_edit_template(id, species))
}

#[get("/species/new")]
pub fn add_species(data: &State<EditorData>) -> Option<Template> {
    let mut data = data.data.lock().expect("lock shared data");

    let id = data.species_manager.create();

    println!("Create species {}", id.id());

    data.species_manager
        .get(id)
        .map(|species| get_edit_template(id.id(), species))
}

#[derive(FromForm, Debug)]
pub struct SpeciesUpdate<'r> {
    name: &'r str,
    gender_option: &'r str,
}

#[post("/species/update/<id>", data = "<update>")]
pub fn update_species(
    data: &State<EditorData>,
    id: usize,
    update: Form<SpeciesUpdate<'_>>,
) -> Option<Template> {
    let mut data = data.data.lock().expect("lock shared data");

    println!("Update species {} with {:?}", id, update);

    data.species_manager
        .get_mut(SpeciesId::new(id))
        .map(|species| {
            species.set_name(update.name.trim().to_string());
            species.set_gender_option(update.gender_option.into());
            species
        })
        .map(|species| get_details_template(id, species))
}

fn get_details_template(id: usize, species: &Species) -> Template {
    Template::render(
        "species/details",
        context! {
            name: species.name(),
            id: id,
            gender_option: format!("{:?}", species.gender_option()),
        },
    )
}

fn get_edit_template(id: usize, species: &Species) -> Template {
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
