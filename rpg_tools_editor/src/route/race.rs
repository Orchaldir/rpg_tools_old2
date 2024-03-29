use crate::io::write;
use crate::route::get_failed_delete_template;
use crate::EditorData;
use rocket::form::Form;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use rpg_tools_core::model::race::gender::GenderOption;
use rpg_tools_core::model::race::{Race, RaceId};
use rpg_tools_core::model::RpgData;
use rpg_tools_core::usecase::delete::race::delete_race;
use rpg_tools_core::usecase::delete::{BlockingReason, DeleteResult};
use rpg_tools_core::usecase::edit::race::{update_gender_option, update_race_name};
use rpg_tools_core::utils::storage::{Element, Id};
use std::sync::MutexGuard;

pub const RACES_FILE: &str = "races.yaml";

#[get("/race/all")]
pub fn get_all_races(data: &State<EditorData>) -> Template {
    let data = data.data.lock().expect("lock shared data");

    get_all_template(data)
}

#[get("/race/details/<id>")]
pub fn get_race_details(data: &State<EditorData>, id: usize) -> Option<Template> {
    let data = data.data.lock().expect("lock shared data");

    data.race_manager
        .get(RaceId::new(id))
        .map(|race| get_details_template(&data, id, race))
}

#[get("/race/edit/<id>")]
pub fn edit_race(data: &State<EditorData>, id: usize) -> Option<Template> {
    let data = data.data.lock().expect("lock shared data");
    data.race_manager
        .get(RaceId::new(id))
        .map(|race| get_edit_template(id, race, "", ""))
}

#[get("/race/new")]
pub fn add_race(data: &State<EditorData>) -> Option<Template> {
    let mut data = data.data.lock().expect("lock shared data");

    let id = data.race_manager.create();

    println!("Create race {}", id.id());

    data.race_manager
        .get(id)
        .map(|race| get_edit_template(id.id(), race, "", ""))
}

#[derive(FromForm, Debug)]
pub struct RaceUpdate<'r> {
    name: &'r str,
    gender_option: &'r str,
}

#[post("/race/update/<id>", data = "<update>")]
pub fn update_race(
    data: &State<EditorData>,
    id: usize,
    update: Form<RaceUpdate<'_>>,
) -> Option<Template> {
    let mut data = data.data.lock().expect("lock shared data");

    println!("Update race {} with {:?}", id, update);

    let race_id = RaceId::new(id);

    if let Err(e) = update_race_name(&mut data, race_id, update.name) {
        return data
            .race_manager
            .get(race_id)
            .map(|race| get_edit_template(id, race, &e.to_string(), ""));
    }

    if let Err(e) = update_gender_option(&mut data, race_id, update.gender_option.into()) {
        return data
            .race_manager
            .get(race_id)
            .map(|race| get_edit_template(id, race, "", &e.to_string()));
    }

    let result = data
        .race_manager
        .get(race_id)
        .map(|race| get_details_template(&data, id, race));

    save_races(&data);

    result
}

#[get("/race/delete/<id>")]
pub fn delete_race_route(data: &State<EditorData>, id: usize) -> Template {
    let mut data = data.data.lock().expect("lock shared data");

    println!("Delete race {}", id);

    let race_id = RaceId::new(id);
    let result = delete_race(&mut data, race_id);
    let name = data
        .race_manager
        .get(race_id)
        .map(|race| race.name())
        .unwrap_or("Unknown")
        .to_string();

    match result {
        DeleteResult::Ok => {
            save_races(&data);
            get_all_template(data)
        }
        DeleteResult::NotFound => {
            get_failed_delete_template(data, "race", id, &name, BlockingReason::default())
        }
        DeleteResult::Blocked(reason) => {
            get_failed_delete_template(data, "race", id, &name, reason)
        }
    }
}

fn get_all_template(data: MutexGuard<RpgData>) -> Template {
    let races: Vec<(usize, &str)> = data
        .race_manager
        .get_all()
        .iter()
        .map(|r| (r.id().id(), r.name()))
        .collect();

    Template::render(
        "race/all",
        context! {
            races: races,
        },
    )
}

fn get_details_template(data: &RpgData, id: usize, race: &Race) -> Template {
    let characters: Vec<(usize, &str)> = data
        .character_manager
        .get_all()
        .iter()
        .filter(|c| c.race().eq(&race.id()))
        .map(|c| (c.id().id(), c.name()))
        .collect();

    Template::render(
        "race/details",
        context! {
            name: race.name(),
            id: id,
            gender_option: format!("{:?}", race.gender_option()),
            characters: characters,
        },
    )
}

fn get_edit_template(id: usize, race: &Race, name_error: &str, gender_error: &str) -> Template {
    Template::render(
        "race/edit",
        context! {
            id: id,
            name: race.name(),
            name_error: name_error,
            gender_options: GenderOption::get_all(),
            gender_option: race.gender_option(),
            gender_error: gender_error,
        },
    )
}

fn save_races(data: &RpgData) {
    if let Err(e) = write(data.race_manager.get_all(), &data.get_path(RACES_FILE)) {
        println!("Failed to save the races: {}", e);
    }
}
