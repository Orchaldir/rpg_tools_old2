use crate::EditorData;
use rocket::State;
use rocket_dyn_templates::{context, Template};

pub const CHARACTER_FILE: &str = "resources/characters/characters.yaml";

#[get("/character/all")]
pub fn get_all_characters(data: &State<EditorData>) -> Template {
    let data = data.data.lock().expect("lock shared data");
    let characters: Vec<(usize, &str)> = data
        .character_manager
        .get_all()
        .iter()
        .map(|c| (c.id().id(), c.name()))
        .collect();
    let total = characters.len();

    Template::render(
        "character/all",
        context! {
            characters: characters,
            total: total,
        },
    )
}
