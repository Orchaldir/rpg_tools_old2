use crate::EditorData;
use rocket::State;
use rocket_dyn_templates::{context, Template};

#[get("/species")]
pub fn get_all_species(data: &State<EditorData>) -> Template {
    let data = data.data.lock().expect("lock shared data");

    Template::render(
        "species/all",
        context! {
            total: data.species_manager.get_all().len(),
        },
    )
}
