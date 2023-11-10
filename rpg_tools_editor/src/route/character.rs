use crate::io::write;
use crate::EditorData;
use rocket::form::Form;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use rpg_tools_core::model::character::gender::Gender;
use rpg_tools_core::model::character::{Character, CharacterId};
use rpg_tools_core::model::RpgData;
use std::path::Path;

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

#[get("/character/details/<id>")]
pub fn get_character_details(data: &State<EditorData>, id: usize) -> Option<Template> {
    let data = data.data.lock().expect("lock shared data");
    data.character_manager
        .get(CharacterId::new(id))
        .map(|character| get_details_template(&data, id, character))
}

#[get("/character/edit/<id>")]
pub fn edit_character(data: &State<EditorData>, id: usize) -> Option<Template> {
    let data = data.data.lock().expect("lock shared data");
    data.character_manager
        .get(CharacterId::new(id))
        .map(|character| get_edit_template(&data, id, character))
}

#[get("/character/new")]
pub fn add_character(data: &State<EditorData>) -> Option<Template> {
    let mut data = data.data.lock().expect("lock shared data");

    let id = data.character_manager.create();

    println!("Create character {}", id.id());

    data.character_manager
        .get(id)
        .map(|character| get_edit_template(&data, id.id(), character))
}

#[derive(FromForm, Debug)]
pub struct CharacterUpdate<'r> {
    name: &'r str,
    race: &'r str,
    gender: &'r str,
}

#[post("/character/update/<id>", data = "<update>")]
pub fn update_character(
    data: &State<EditorData>,
    id: usize,
    update: Form<CharacterUpdate<'_>>,
) -> Option<Template> {
    let mut data = data.data.lock().expect("lock shared data");

    println!("Update character {} with {:?}", id, update);

    let race = data
        .race_manager
        .get_all()
        .iter()
        .find(|race| race.name().eq(update.race))
        .map(|race| *race.id());

    data.character_manager
        .get_mut(CharacterId::new(id))
        .map(|character| {
            character.set_name(update.name.trim().to_string());

            if let Some(id) = race {
                character.set_race(id);
            }

            character.set_gender(update.gender.into());
            character
        });

    save_and_show_character(&data, id)
}

fn get_details_template(data: &RpgData, id: usize, character: &Character) -> Template {
    let race = data
        .race_manager
        .get(character.race())
        .map(|race| race.name())
        .unwrap_or("Unknown");

    Template::render(
        "character/details",
        context! {
            id: id,
            name: character.name(),
            race_id: character.race(),
            race: race,
            gender: character.gender(),
            appearance: character.appearance(),
        },
    )
}

fn get_edit_template(data: &RpgData, id: usize, character: &Character) -> Template {
    let races: Vec<&str> = data
        .race_manager
        .get_all()
        .iter()
        .map(|race| race.name())
        .collect();
    let race = data
        .race_manager
        .get(character.race())
        .map(|race| race.name())
        .unwrap_or("Unknown");

    Template::render(
        "character/edit",
        context! {
            id: id,
            name: character.name(),
            races: races,
            race: race,
            genders: Gender::get_all(),
            gender: character.gender(),
        },
    )
}

pub fn save_and_show_character(data: &RpgData, id: usize) -> Option<Template> {
    let result = data
        .character_manager
        .get(CharacterId::new(id))
        .map(|character| get_details_template(data, id, character));

    if let Err(e) = write(data.character_manager.get_all(), Path::new(CHARACTER_FILE)) {
        println!("Failed to save the characters: {}", e);
    }

    result
}
