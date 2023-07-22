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
use rpg_tools_core::model::character::CharacterId;
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

#[get("/character")]
fn get_characters(data: &State<EditorData>) -> Template {
    let data = data.data.lock().expect("lock shared data");
    let characters: Vec<(usize, &str)> = data
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

#[get("/character/<id>")]
fn get_character(data: &State<EditorData>, id: usize) -> Option<Template> {
    let data = data.data.lock().expect("lock shared data");
    data.get(CharacterId::new(id)).map(|character| {
        Template::render(
            "character",
            context! {
                id: id,
                name: character.name(),
            },
        )
    })
}

#[rocket::main]
async fn main() -> Result<()> {
    if let Err(e) = rocket::build()
        .manage(EditorData {
            data: Mutex::new(init()),
        })
        .mount("/", routes![home, get_characters, get_character])
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

    for skin in &[
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
                skin: *skin,
            },
            Head::default(),
            Length::from_metre(1.5),
        ))
    }

    manager
}
