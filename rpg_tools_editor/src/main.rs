extern crate macro_core;
#[macro_use]
extern crate rocket;

use crate::appearance::{apply_update_to_appearance, render_to_svg, RawSvg};
use crate::io::{read, write};
use anyhow::Result;
use rocket::form::Form;
use rocket::fs::FileServer;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::character::gender::Gender;
use rpg_tools_core::model::character::manager::CharacterMgr;
use rpg_tools_core::model::character::{Character, CharacterId};
use rpg_tools_core::model::RpgData;
use rpg_tools_rendering::rendering::config::example::create_config;
use rpg_tools_rendering::rendering::config::RenderConfig;
use std::path::Path;
use std::sync::Mutex;

pub mod appearance;
pub mod io;
pub mod parser;

const FILE: &str = "resources/characters/characters.yaml";

struct EditorData {
    config: RenderConfig,
    data: Mutex<RpgData>,
    preview: Mutex<Appearance>,
}

#[get("/")]
fn home(data: &State<EditorData>) -> Template {
    let data = data.data.lock().expect("lock shared data");
    Template::render(
        "home",
        context! {
            species: data.species_manager.get_all().len(),
            characters: data.character_manager.get_all().len(),
        },
    )
}

#[get("/character")]
fn get_characters(data: &State<EditorData>) -> Template {
    let data = data.data.lock().expect("lock shared data");
    let characters: Vec<(usize, &str)> = data
        .character_manager
        .get_all()
        .iter()
        .map(|c| (c.id().id(), c.name()))
        .collect();
    let total = characters.len();

    Template::render(
        "characters",
        context! {
            characters: characters,
            total: total,
        },
    )
}

#[get("/character/new")]
fn add_character(data: &State<EditorData>) -> Option<Template> {
    let mut data = data.data.lock().expect("lock shared data");

    let id = data.character_manager.create();

    println!("Create character {}", id.id());

    data.character_manager
        .get(id)
        .map(|character| edit_character_template(id.id(), character))
}

#[get("/character/<id>")]
fn get_character(data: &State<EditorData>, id: usize) -> Option<Template> {
    let data = data.data.lock().expect("lock shared data");
    data.character_manager
        .get(CharacterId::new(id))
        .map(|character| show_character_template(id, character))
}

#[get("/character/<id>/edit")]
fn edit_character(data: &State<EditorData>, id: usize) -> Option<Template> {
    let data = data.data.lock().expect("lock shared data");
    data.character_manager
        .get(CharacterId::new(id))
        .map(|character| edit_character_template(id, character))
}

#[derive(FromForm, Debug)]
struct CharacterUpdate<'r> {
    name: &'r str,
    gender: &'r str,
}

#[post("/character/<id>/update", data = "<update>")]
fn update_character(
    data: &State<EditorData>,
    id: usize,
    update: Form<CharacterUpdate<'_>>,
) -> Option<Template> {
    let mut data = data.data.lock().expect("lock shared data");

    println!("Update character {} with {:?}", id, update);

    data.character_manager
        .get_mut(CharacterId::new(id))
        .map(|character| {
            character.set_name(update.name.trim().to_string());
            character.set_gender(update.gender.into());
            character
        })
        .map(|character| show_character_template(id, character))
}

#[get("/character/<id>/front.svg")]
fn get_front(state: &State<EditorData>, id: usize) -> Option<RawSvg> {
    let data = state.data.lock().expect("lock shared data");
    data.character_manager
        .get(CharacterId::new(id))
        .map(|character| render_to_svg(&state.config, character.appearance(), true))
}

#[get("/character/<id>/back.svg")]
fn get_back(state: &State<EditorData>, id: usize) -> Option<RawSvg> {
    let data = state.data.lock().expect("lock shared data");
    data.character_manager
        .get(CharacterId::new(id))
        .map(|character| render_to_svg(&state.config, character.appearance(), false))
}

#[get("/appearance/preview/front.svg")]
fn get_preview_front(state: &State<EditorData>) -> RawSvg {
    let preview = state.preview.lock().expect("lock shared preview");
    render_to_svg(&state.config, &preview, true)
}

#[get("/appearance/preview/back.svg")]
fn get_preview_back(state: &State<EditorData>) -> RawSvg {
    let preview = state.preview.lock().expect("lock shared preview");
    render_to_svg(&state.config, &preview, false)
}

#[get("/appearance/<id>/edit")]
fn edit_appearance(state: &State<EditorData>, id: usize) -> Option<Template> {
    let data = state.data.lock().expect("lock shared data");
    let mut preview = state.preview.lock().expect("lock shared preview");

    data.character_manager
        .get(CharacterId::new(id))
        .map(|character| {
            *preview = *character.appearance();

            edit_appearance_template(character, &preview)
        })
}

#[post("/appearance/<id>/update", data = "<update>")]
fn update_appearance(data: &State<EditorData>, id: usize, update: String) -> Option<Template> {
    let mut data = data.data.lock().expect("lock shared data");

    println!("Update appearance of character {} with {:?}", id, update);

    let result = data
        .character_manager
        .get_mut(CharacterId::new(id))
        .map(|character| {
            character.set_appearance(apply_update_to_appearance(&update));
            character
        })
        .map(|character| show_character_template(id, character));

    if let Err(e) = write(data.character_manager.get_all(), Path::new(FILE)) {
        println!("Failed to save the characters: {}", e);
    }

    result
}

#[post("/appearance/<id>/preview", data = "<update>")]
fn update_appearance_preview(
    state: &State<EditorData>,
    id: usize,
    update: String,
) -> Option<Template> {
    let data = state.data.lock().expect("lock shared data");
    let mut preview = state.preview.lock().expect("lock shared preview");

    println!("Preview appearance of character {} with {:?}", id, update);

    data.character_manager
        .get(CharacterId::new(id))
        .map(|character| {
            *preview = apply_update_to_appearance(&update);

            edit_appearance_template(character, &preview)
        })
}

fn show_character_template(id: usize, character: &Character) -> Template {
    Template::render(
        "character",
        context! {
            id: id,
            name: character.name(),
            gender: character.gender(),
            appearance: character.appearance(),
        },
    )
}

fn edit_character_template(id: usize, character: &Character) -> Template {
    Template::render(
        "character_edit",
        context! {
            id: id,
            name: character.name(),
            genders: Gender::get_all(),
            gender: character.gender(),
        },
    )
}

fn edit_appearance_template(character: &Character, appearance: &Appearance) -> Template {
    Template::render(
        "appearance_edit",
        context! {
            id: character.id().id(),
            name: character.name(),
            appearance: appearance,
        },
    )
}

#[rocket::main]
async fn main() -> Result<()> {
    if let Err(e) = rocket::build()
        .manage(EditorData {
            config: create_config(),
            data: Mutex::new(init()),
            preview: Mutex::new(Appearance::default()),
        })
        .mount("/static", FileServer::from("rpg_tools_editor/static/"))
        .mount(
            "/",
            routes![
                home,
                get_characters,
                add_character,
                get_character,
                edit_character,
                update_character,
                edit_appearance,
                update_appearance,
                get_preview_front,
                get_preview_back,
                update_appearance_preview,
                get_front,
                get_back,
            ],
        )
        .attach(Template::fairing())
        .launch()
        .await
    {
        drop(e);
    };

    Ok(())
}

fn init() -> RpgData {
    let characters: Result<Vec<Character>> = read(Path::new(FILE));

    let character_manager = match characters {
        Ok(characters) => {
            println!("Loaded {} characters.", characters.len());
            CharacterMgr::new(characters)
        }
        Err(e) => {
            println!("Failed to load the characters: {}", e);
            CharacterMgr::default()
        }
    };

    RpgData {
        species_manager: Default::default(),
        character_manager,
    }
}
