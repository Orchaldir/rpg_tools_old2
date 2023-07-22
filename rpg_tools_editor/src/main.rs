#[macro_use]
extern crate rocket;

use anyhow::Result;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use rpg_tools_core::model::character::appearance::body::{Body, BodyShape};
use rpg_tools_core::model::character::appearance::head::Head;
use rpg_tools_core::model::character::appearance::skin::{Skin, SkinColor};
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::character::manager::CharacterMgr;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;
use rpg_tools_core::model::width::Width;
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
            data: Mutex::new(init()),
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

fn init() -> CharacterMgr {
    let mut manager = CharacterMgr::default();

    for skin in vec![
        Skin::Skin(SkinColor::Fair),
        Skin::Skin(SkinColor::Light),
        Skin::Skin(SkinColor::Medium),
        Skin::Skin(SkinColor::Tan),
        Skin::Skin(SkinColor::Dark),
        Skin::Skin(SkinColor::VeryDark),
        Skin::Skin(SkinColor::Exotic(Color::Green)),
    ] {
        let id = manager.create();
        let character = manager.get_mut(id).unwrap();

        character.set_appearance(Appearance::humanoid(
            Body {
                shape: BodyShape::Rectangle,
                width: Width::Average,
                skin,
            },
            Head::default(),
            Length::from_metre(1.5),
        ))
    }

    manager
}
