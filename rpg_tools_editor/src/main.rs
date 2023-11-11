extern crate macro_core;
#[macro_use]
extern crate rocket;

use crate::io::read;
use crate::route::appearance::{
    edit_appearance, get_appearance_back, get_appearance_front, get_preview_back,
    get_preview_front, update_appearance, update_appearance_preview,
};
use crate::route::character::{
    add_character, edit_character, get_all_characters, get_character_details, update_character,
    CHARACTER_FILE,
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
use rpg_tools_core::model::character::Character;
use rpg_tools_core::model::race::manager::RaceMgr;
use rpg_tools_core::model::race::Race;
use rpg_tools_core::model::RpgData;
use rpg_tools_rendering::rendering::config::example::create_config;
use rpg_tools_rendering::rendering::config::RenderConfig;
use std::path::Path;
use std::sync::Mutex;

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
                get_appearance_front,
                get_appearance_back,
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
