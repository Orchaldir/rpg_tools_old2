use crate::EditorData;
use rocket::State;
use rocket_dyn_templates::{context, Template};

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
