#[macro_use]
extern crate rocket;

use anyhow::Result;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use rpg_tools_core::model::character::manager::CharacterMgr;
use std::sync::Mutex;

struct EditorData {
    data: Mutex<CharacterMgr>,
}

#[get("/")]
fn home(data: &State<EditorData>) -> Template {
    let data = data.data.lock().expect("lock shared data");
    Template::render(
        "home",
        context! {
            characters: data.get_all().len(),
        },
    )
}

#[rocket::main]
async fn main() -> Result<()> {
    if let Err(e) = rocket::build()
        .manage(EditorData {
            data: Mutex::new(CharacterMgr::default()),
        })
        .mount("/", routes![home])
        .attach(Template::fairing())
        .launch()
        .await
    {
        drop(e);
    };

    Ok(())
}
