extern crate macro_core;
#[macro_use]
extern crate rocket;

use crate::appearance::{apply_update_to_appearance, render_to_svg, RawSvg};
use crate::io::read;
use crate::route::character::{
    add_character, edit_character, get_all_characters, get_character_details,
    save_and_show_character, update_character, CHARACTER_FILE,
};
use crate::route::race::{
    add_race, edit_race, get_all_races, get_race_details, update_race, RACES_FILE,
};
use anyhow::Result;
use rocket::fs::FileServer;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::character::manager::CharacterMgr;
use rpg_tools_core::model::character::{Character, CharacterId};
use rpg_tools_core::model::race::manager::RaceMgr;
use rpg_tools_core::model::race::Race;
use rpg_tools_core::model::RpgData;
use rpg_tools_rendering::rendering::config::example::create_config;
use rpg_tools_rendering::rendering::config::RenderConfig;
use std::path::Path;
use std::sync::Mutex;

pub mod appearance;
pub mod io;
pub mod parser;
pub mod route;

pub struct EditorData {
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
            races: data.race_manager.get_all().len(),
            characters: data.character_manager.get_all().len(),
        },
    )
}

#[get("/appearance/<id>/front.svg")]
fn get_front(state: &State<EditorData>, id: usize) -> Option<RawSvg> {
    let data = state.data.lock().expect("lock shared data");
    data.character_manager
        .get(CharacterId::new(id))
        .map(|character| render_to_svg(&state.config, character.appearance(), true))
}

#[get("/appearance/<id>/back.svg")]
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

    data.character_manager
        .get_mut(CharacterId::new(id))
        .map(|character| {
            character.set_appearance(apply_update_to_appearance(&update));
            character
        });

    save_and_show_character(&data, id)
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
                get_all_characters,
                add_character,
                get_character_details,
                edit_character,
                update_character,
                edit_appearance,
                update_appearance,
                get_preview_front,
                get_preview_back,
                update_appearance_preview,
                get_front,
                get_back,
                get_all_races,
                get_race_details,
                add_race,
                edit_race,
                update_race,
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
    let races: Result<Vec<Race>> = read(Path::new(RACES_FILE));

    let race_manager = match races {
        Ok(races) => {
            println!("Loaded {} races.", races.len());
            RaceMgr::new(races)
        }
        Err(e) => {
            println!("Failed to load the races: {}", e);
            return RpgData::default();
        }
    };

    let characters: Result<Vec<Character>> = read(Path::new(CHARACTER_FILE));

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
        character_manager,
        race_manager,
    }
}
