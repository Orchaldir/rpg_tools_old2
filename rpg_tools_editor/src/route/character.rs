use crate::io::write;
use crate::route::get_failed_delete_template;
use crate::route::relation::get_relationships;
use crate::EditorData;
use rocket::form::Form;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use rpg_tools_core::model::character::gender::Gender;
use rpg_tools_core::model::character::{Character, CharacterId};
use rpg_tools_core::model::RpgData;
use rpg_tools_core::usecase::delete::character::delete_character;
use rpg_tools_core::usecase::delete::{BlockingReason, DeleteResult};
use rpg_tools_core::usecase::edit::character::{
    update_character_culture, update_character_gender, update_character_name, update_character_race,
};
use rpg_tools_core::utils::storage::Element;
use rpg_tools_core::utils::storage::Id;
use std::sync::MutexGuard;

pub const CHARACTERS_FILE: &str = "characters.yaml";

#[get("/character/all")]
pub fn get_all_characters(data: &State<EditorData>) -> Template {
    let data = data.data.lock().expect("lock shared data");

    get_all_template(data)
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
        .map(|character| get_edit_template(&data, id, character, "", "", "", ""))
}

#[get("/character/new")]
pub fn add_character(data: &State<EditorData>) -> Option<Template> {
    let mut data = data.data.lock().expect("lock shared data");

    let id = data.character_manager.create();

    println!("Create character {}", id.id());

    data.character_manager
        .get(id)
        .map(|character| get_edit_template(&data, id.id(), character, "", "", "", ""))
}

#[derive(FromForm, Debug)]
pub struct CharacterUpdate<'r> {
    name: &'r str,
    race: &'r str,
    culture: &'r str,
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

    let character_id = CharacterId::new(id);

    if let Err(e) = update_character_name(&mut data, character_id, update.name) {
        return data
            .character_manager
            .get(character_id)
            .map(|character| get_edit_template(&data, id, character, &e.to_string(), "", "", ""));
    }

    if let Err(e) = update_character_gender(&mut data, character_id, update.gender.into()) {
        return data
            .character_manager
            .get(character_id)
            .map(|character| get_edit_template(&data, id, character, "", &e.to_string(), "", ""));
    }

    if let Err(e) = update_character_race(&mut data, character_id, update.race) {
        return data
            .character_manager
            .get(character_id)
            .map(|character| get_edit_template(&data, id, character, "", "", &e.to_string(), ""));
    }

    if let Err(e) = update_character_culture(&mut data, character_id, update.culture) {
        return data
            .character_manager
            .get(character_id)
            .map(|character| get_edit_template(&data, id, character, "", "", "", &e.to_string()));
    }

    let race = data
        .race_manager
        .get_all()
        .iter()
        .find(|race| race.name().eq(update.race))
        .map(|race| race.id());

    data.character_manager
        .get_mut(CharacterId::new(id))
        .map(|character| {
            if let Some(id) = race {
                character.set_race(id);
            }
            character
        });

    save_and_show_character(&data, id)
}

#[get("/character/delete/<id>")]
pub fn delete_character_route(data: &State<EditorData>, id: usize) -> Template {
    let mut data = data.data.lock().expect("lock shared data");

    println!("Delete character {}", id);

    let character_id = CharacterId::new(id);
    let result = delete_character(&mut data, character_id);
    let name = data
        .character_manager
        .get(character_id)
        .map(|character| character.name())
        .unwrap_or("Unknown")
        .to_string();

    match result {
        DeleteResult::Ok => {
            write_characters(&data);
            get_all_template(data)
        }
        DeleteResult::NotFound => {
            get_failed_delete_template(data, "character", id, &name, BlockingReason::default())
        }
        DeleteResult::Blocked(reason) => {
            get_failed_delete_template(data, "character", id, &name, reason)
        }
    }
}

fn get_all_template(data: MutexGuard<RpgData>) -> Template {
    let characters: Vec<(usize, &str)> = data
        .character_manager
        .get_all()
        .iter()
        .map(|c| (c.id().id(), c.name()))
        .collect();

    Template::render(
        "character/all",
        context! {
            characters: characters,
        },
    )
}

fn get_details_template(data: &RpgData, id: usize, character: &Character) -> Template {
    let race = data
        .race_manager
        .get(character.race())
        .map(|race| race.name())
        .unwrap_or("Unknown");
    let culture = data
        .culture_manager
        .get(character.culture())
        .map(|c| c.name())
        .unwrap_or("Unknown");

    Template::render(
        "character/details",
        context! {
            id: id,
            name: character.name(),
            race_id: character.race(),
            race: race,
            culture_id: character.culture(),
            culture: culture,
            gender: character.gender(),
            appearance: character.appearance(),
            relationships: get_relationships(data, &data.relations.relationships, character.id()),
        },
    )
}

fn get_edit_template(
    data: &RpgData,
    id: usize,
    character: &Character,
    name_error: &str,
    gender_error: &str,
    race_error: &str,
    culture_error: &str,
) -> Template {
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
    let cultures: Vec<&str> = data
        .culture_manager
        .get_all()
        .iter()
        .map(|culture| culture.name())
        .collect();
    let culture = data
        .culture_manager
        .get(character.culture())
        .map(|culture| culture.name())
        .unwrap_or("Unknown");

    Template::render(
        "character/edit",
        context! {
            id: id,
            name: character.name(),
            name_error: name_error,
            races: races,
            race: race,
            race_error: race_error,
            genders: Gender::get_all(),
            gender: character.gender(),
            gender_error: gender_error,
            cultures: cultures,
            culture: culture,
            culture_error: culture_error,
        },
    )
}

pub fn save_and_show_character(data: &RpgData, id: usize) -> Option<Template> {
    let result = data
        .character_manager
        .get(CharacterId::new(id))
        .map(|character| get_details_template(data, id, character));

    write_characters(data);

    result
}

fn write_characters(data: &RpgData) {
    if let Err(e) = write(
        data.character_manager.get_all(),
        &data.get_path(CHARACTERS_FILE),
    ) {
        println!("Failed to save the characters: {}", e);
    }
}
