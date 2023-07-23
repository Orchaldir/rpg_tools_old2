#[macro_use]
extern crate rocket;

use crate::appearance::{apply_update_to_appearance, render_to_svg, AppearanceOptions, RawSvg};
use anyhow::Result;
use rocket::form::Form;
use rocket::fs::FileServer;
use rocket::http::Status;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use rpg_tools_core::model::character::appearance::body::{Body, BodyShape};
use rpg_tools_core::model::character::appearance::ear::Ears;
use rpg_tools_core::model::character::appearance::eye::Eyes;
use rpg_tools_core::model::character::appearance::hair::Hair;
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape};
use rpg_tools_core::model::character::appearance::mouth::Mouth;
use rpg_tools_core::model::character::appearance::skin::{Skin, SkinColor};
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::character::manager::CharacterMgr;
use rpg_tools_core::model::character::{Character, CharacterId};
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;
use rpg_tools_core::model::width::Width;
use rpg_tools_rendering::rendering::config::example::create_config;
use rpg_tools_rendering::rendering::config::RenderConfig;
use std::sync::Mutex;

pub mod appearance;

struct EditorData {
    config: RenderConfig,
    data: Mutex<CharacterMgr>,
    preview: Mutex<Appearance>,
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

#[get("/character/new")]
fn add_character(data: &State<EditorData>) -> Option<Template> {
    let mut data = data.data.lock().expect("lock shared data");

    let id = data.create();

    println!("Create character {}", id.id());

    data.get_mut(id)
        .map(|character| edit_character_template(id.id(), character))
}

#[get("/character/<id>")]
fn get_character(data: &State<EditorData>, id: usize) -> Option<Template> {
    let data = data.data.lock().expect("lock shared data");
    data.get(CharacterId::new(id))
        .map(|character| show_character_template(id, character))
}

#[get("/character/<id>/edit")]
fn edit_character(data: &State<EditorData>, id: usize) -> Option<Template> {
    let data = data.data.lock().expect("lock shared data");
    data.get(CharacterId::new(id))
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

    data.get_mut(CharacterId::new(id))
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
    data.get(CharacterId::new(id))
        .map(|character| render_to_svg(&state.config, character.appearance()))
}

#[get("/appearance/preview/<time>/front.svg")]
fn get_appearance_preview(state: &State<EditorData>, time: usize) -> RawSvg {
    let preview = state.preview.lock().expect("lock shared preview");
    render_to_svg(&state.config, &preview)
}

#[get("/appearance/<id>/edit")]
fn edit_appearance(state: &State<EditorData>, id: usize) -> Option<Template> {
    let data = state.data.lock().expect("lock shared data");
    let mut preview = state.preview.lock().expect("lock shared preview");

    data.get(CharacterId::new(id)).map(|character| {
        *preview = *character.appearance();

        edit_appearance_template(character, &preview)
    })
}

#[post("/appearance/<id>/update", data = "<update>")]
fn update_appearance(data: &State<EditorData>, id: usize, update: String) -> Option<Template> {
    let mut data = data.data.lock().expect("lock shared data");

    println!("Update appearance of character {} with {:?}", id, update);

    data.get_mut(CharacterId::new(id))
        .map(|character| {
            character.set_appearance(apply_update_to_appearance(character.appearance(), &update));
            character
        })
        .map(|character| show_character_template(id, character))
}

#[post("/appearance/<id>/preview", data = "<update>")]
fn update_appearance_preview(
    state: &State<EditorData>,
    id: usize,
    update: String,
) -> Option<Template> {
    let mut data = state.data.lock().expect("lock shared data");
    let mut preview = state.preview.lock().expect("lock shared preview");

    println!("Preview appearance of character {} with {:?}", id, update);

    data.get_mut(CharacterId::new(id)).map(|character| {
        *preview = apply_update_to_appearance(character.appearance(), &update);

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
            genders: vec!["Female", "Genderless", "Male"],
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
            options: AppearanceOptions::new(),
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
                get_appearance_preview,
                update_appearance_preview,
                get_front
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

fn init() -> CharacterMgr {
    let mut manager = CharacterMgr::default();

    for skin in &[
        Skin::Skin(SkinColor::Fair),
        Skin::Skin(SkinColor::Light),
        Skin::Skin(SkinColor::Medium),
        Skin::Skin(SkinColor::Tan),
        Skin::Skin(SkinColor::Dark),
        Skin::Skin(SkinColor::VeryDark),
        Skin::ExoticSkin(Color::Green),
    ] {
        let id = manager.create();
        let character = manager.get_mut(id).unwrap();

        character.set_appearance(Appearance::humanoid(
            Body {
                shape: BodyShape::Rectangle,
                width: Width::Average,
                skin: *skin,
            },
            Head {
                ears: Ears::None,
                eyes: Eyes::None,
                hair: Hair::None,
                mouth: Mouth::None,
                shape: HeadShape::default(),
                skin: *skin,
            },
            Length::from_metre(1.5),
        ))
    }

    manager
}
