extern crate macro_core;
#[macro_use]
extern crate rocket;

use crate::io::{get_path, read};
use crate::route::appearance::{
    edit_appearance, get_appearance_back, get_appearance_front, get_preview_back,
    get_preview_front, update_appearance, update_appearance_preview,
};
use crate::route::character::{
    add_character, edit_character, get_all_characters, get_character_details, update_character,
    CHARACTERS_FILE,
};
use crate::route::culture::{
    add_culture, edit_culture, get_all_cultures, get_culture_details, update_culture, CULTURES_FILE,
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
use rpg_tools_core::model::culture::manager::CultureMgr;
use rpg_tools_core::model::culture::Culture;
use rpg_tools_core::model::race::manager::RaceMgr;
use rpg_tools_core::model::race::Race;
use rpg_tools_core::model::RpgData;
use rpg_tools_rendering::rendering::config::example::create_config;
use rpg_tools_rendering::rendering::config::RenderConfig;
use std::path::PathBuf;
use std::sync::Mutex;

pub mod io;
pub mod parser;
pub mod route;

pub struct EditorData {
    config: RenderConfig,
    data: Mutex<RpgData>,
    preview: Mutex<Appearance>,
    path: String,
}

impl EditorData {
    pub fn get_path(&self, file: &str) -> PathBuf {
        get_path(&self.path, file)
    }
}

#[get("/")]
fn home(data: &State<EditorData>) -> Template {
    let data = data.data.lock().expect("lock shared data");
    Template::render(
        "home",
        context! {
            cultures: data.culture_manager.get_all().len(),
            races: data.race_manager.get_all().len(),
            characters: data.character_manager.get_all().len(),
        },
    )
}

#[rocket::main]
async fn main() -> Result<()> {
    let setting_path = "resources/settings/eberron";

    if let Err(e) = rocket::build()
        .manage(EditorData {
            config: create_config(),
            data: Mutex::new(init(setting_path)),
            preview: Mutex::new(Appearance::default()),
            path: setting_path.to_string(),
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
                get_all_cultures,
                get_culture_details,
                add_culture,
                edit_culture,
                update_culture,
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

fn init(setting_path: &str) -> RpgData {
    let cultures: Result<Vec<Culture>> = read(&get_path(setting_path, CULTURES_FILE));

    let culture_manager = match cultures {
        Ok(cultures) => {
            println!("Loaded {} cultures.", cultures.len());
            CultureMgr::new(cultures)
        }
        Err(e) => {
            println!("Failed to load the cultures: {}", e);
            return RpgData::default();
        }
    };

    let races: Result<Vec<Race>> = read(&get_path(setting_path, RACES_FILE));

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

    let characters: Result<Vec<Character>> = read(&get_path(setting_path, CHARACTERS_FILE));

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
        culture_manager,
        race_manager,
    }
}
