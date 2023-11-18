extern crate macro_core;
#[macro_use]
extern crate rocket;

use crate::io::read;
use crate::route::appearance::{
    edit_appearance, get_appearance_back, get_appearance_front, get_preview_back,
    get_preview_front, update_appearance, update_appearance_preview,
};
use crate::route::character::{
    add_character, delete_character_route, edit_character, get_all_characters,
    get_character_details, update_character, CHARACTERS_FILE,
};
use crate::route::culture::{
    add_culture, delete_culture_route, edit_culture, get_all_cultures, get_culture_details,
    update_culture, CULTURES_FILE,
};
use crate::route::race::{
    add_race, delete_race_route, edit_race, get_all_races, get_race_details, update_race,
    RACES_FILE,
};
use anyhow::{bail, Result};
use rocket::fs::FileServer;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::character::relation::relationship::RelationshipType::Friend;
use rpg_tools_core::model::character::{Character, CharacterId};
use rpg_tools_core::model::culture::Culture;
use rpg_tools_core::model::race::Race;
use rpg_tools_core::model::{get_setting_path, Relations, RpgData};
use rpg_tools_core::utils::storage::{Id, Storage};
use rpg_tools_rendering::rendering::config::example::create_config;
use rpg_tools_rendering::rendering::config::RenderConfig;
use std::env;
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
            cultures: data.culture_manager.get_all().len(),
            races: data.race_manager.get_all().len(),
            characters: data.character_manager.get_all().len(),
        },
    )
}

#[rocket::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        bail!("Setting argument missing!");
    }

    let setting = &args[1];

    if let Err(e) = rocket::build()
        .manage(EditorData {
            config: create_config(),
            data: Mutex::new(init(setting)),
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
                delete_character_route,
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
                delete_race_route,
                get_all_cultures,
                get_culture_details,
                add_culture,
                edit_culture,
                update_culture,
                delete_culture_route,
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

fn init(setting: &str) -> RpgData {
    println!("Load setting '{}'", setting);

    let cultures: Result<Vec<Culture>> = read(&get_setting_path(setting, CULTURES_FILE));

    let culture_manager = match cultures {
        Ok(cultures) => {
            println!("Loaded {} cultures.", cultures.len());
            Storage::new(cultures)
        }
        Err(e) => {
            println!("Failed to load the cultures: {}", e);
            return RpgData::empty(setting.to_string());
        }
    };

    let races: Result<Vec<Race>> = read(&get_setting_path(setting, RACES_FILE));

    let race_manager = match races {
        Ok(races) => {
            println!("Loaded {} races.", races.len());
            Storage::new(races)
        }
        Err(e) => {
            println!("Failed to load the races: {}", e);
            return RpgData::empty(setting.to_string());
        }
    };

    let characters: Result<Vec<Character>> = read(&get_setting_path(setting, CHARACTERS_FILE));

    let character_manager = match characters {
        Ok(characters) => {
            println!("Loaded {} characters.", characters.len());
            Storage::new(characters)
        }
        Err(e) => {
            println!("Failed to load the characters: {}", e);
            return RpgData::empty(setting.to_string());
        }
    };

    let mut relations = Relations::default();
    relations
        .relationships
        .add(CharacterId::new(0), CharacterId::new(1), Friend);

    RpgData {
        setting: setting.to_string(),
        character_manager,
        culture_manager,
        race_manager,
        relations,
    }
}
