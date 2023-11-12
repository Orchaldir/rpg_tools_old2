use crate::io::write;
use crate::route::get_failed_delete_template;
use crate::EditorData;
use rocket::form::Form;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use rpg_tools_core::model::culture::{Culture, CultureId};
use rpg_tools_core::model::RpgData;
use rpg_tools_core::usecase::delete::culture::delete_culture;
use rpg_tools_core::usecase::delete::DeleteResult;
use rpg_tools_core::usecase::edit::culture::update_culture_name;
use rpg_tools_core::utils::storage::{Entry, Id};
use std::sync::MutexGuard;

pub const CULTURES_FILE: &str = "cultures.yaml";

#[get("/culture/all")]
pub fn get_all_cultures(data: &State<EditorData>) -> Template {
    let data = data.data.lock().expect("lock shared data");

    get_all_template(data)
}

#[get("/culture/details/<id>")]
pub fn get_culture_details(data: &State<EditorData>, id: usize) -> Option<Template> {
    let data = data.data.lock().expect("lock shared data");

    data.culture_manager
        .get(CultureId::new(id))
        .map(|culture| get_details_template(&data, id, culture))
}

#[get("/culture/edit/<id>")]
pub fn edit_culture(data: &State<EditorData>, id: usize) -> Option<Template> {
    let data = data.data.lock().expect("lock shared data");
    data.culture_manager
        .get(CultureId::new(id))
        .map(|culture| get_edit_template(id, culture, ""))
}

#[get("/culture/new")]
pub fn add_culture(data: &State<EditorData>) -> Option<Template> {
    let mut data = data.data.lock().expect("lock shared data");

    let id = data.culture_manager.create();

    println!("Create culture {}", id.id());

    data.culture_manager
        .get(id)
        .map(|culture| get_edit_template(id.id(), culture, ""))
}

#[derive(FromForm, Debug)]
pub struct CultureUpdate<'r> {
    name: &'r str,
}

#[post("/culture/update/<id>", data = "<update>")]
pub fn update_culture(
    data: &State<EditorData>,
    id: usize,
    update: Form<CultureUpdate<'_>>,
) -> Option<Template> {
    let mut data = data.data.lock().expect("lock shared data");

    println!("Update culture {} with {:?}", id, update);

    let culture_id = CultureId::new(id);

    if let Err(e) = update_culture_name(&mut data, culture_id, update.name) {
        return data
            .culture_manager
            .get(culture_id)
            .map(|c| get_edit_template(id, c, &e.to_string()));
    }

    let result = data
        .culture_manager
        .get(culture_id)
        .map(|culture| get_details_template(&data, id, culture));

    save_cultures(&data);

    result
}

#[get("/culture/delete/<id>")]
pub fn delete_culture_route(data: &State<EditorData>, id: usize) -> Template {
    let mut data = data.data.lock().expect("lock shared data");

    println!("Delete culture {}", id);

    let culture_id = CultureId::new(id);
    let result = delete_culture(&mut data, culture_id);

    match result {
        DeleteResult::Ok => {
            save_cultures(&data);
            get_all_template(data)
        }
        _ => {
            let name = data
                .culture_manager
                .get(culture_id)
                .map(|culture| culture.name())
                .unwrap_or("Unknown")
                .to_string();
            get_failed_delete_template(data, "culture", id, &name, result)
        }
    }
}

fn get_all_template(data: MutexGuard<RpgData>) -> Template {
    let cultures: Vec<(usize, &str)> = data
        .culture_manager
        .get_all()
        .iter()
        .map(|r| (r.id().id(), r.name()))
        .collect();

    Template::render(
        "culture/all",
        context! {
            cultures: cultures,
        },
    )
}

fn get_details_template(data: &RpgData, id: usize, culture: &Culture) -> Template {
    let characters: Vec<(usize, &str)> = data
        .character_manager
        .get_all()
        .iter()
        .filter(|c| c.culture().eq(&culture.id()))
        .map(|c| (c.id().id(), c.name()))
        .collect();

    Template::render(
        "culture/details",
        context! {
            name: culture.name(),
            id: id,
            characters: characters,
        },
    )
}

fn get_edit_template(id: usize, culture: &Culture, name_error: &str) -> Template {
    Template::render(
        "culture/edit",
        context! {
            id: id,
            name: culture.name(),
            name_error: name_error,
        },
    )
}

fn save_cultures(data: &RpgData) {
    if let Err(e) = write(
        data.culture_manager.get_all(),
        &data.get_path(CULTURES_FILE),
    ) {
        println!("Failed to save the cultures: {}", e);
    }
}
