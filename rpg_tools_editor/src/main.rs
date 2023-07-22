#[macro_use]
extern crate rocket;

use anyhow::Result;
use rocket::form::Form;
use rocket::fs::FileServer;
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
use rpg_tools_rendering::math::aabb2d::AABB;
use rpg_tools_rendering::renderer::svg::SvgBuilder;
use rpg_tools_rendering::renderer::Renderer;
use rpg_tools_rendering::rendering::character::{calculate_character_size, render_character};
use rpg_tools_rendering::rendering::config::example::{create_border_options, create_config};
use rpg_tools_rendering::rendering::config::RenderConfig;
use std::sync::Mutex;

struct EditorData {
    config: RenderConfig,
    data: Mutex<CharacterMgr>,
}

#[derive(Responder)]
#[response(status = 200, content_type = "image/svg+xml")]
struct RawSvg(String);

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
    data.get(CharacterId::new(id)).map(|character| {
        let size = calculate_character_size(&state.config, character.appearance());
        let aabb = AABB::with_size(size);
        let options = create_border_options();
        let mut svg_builder = SvgBuilder::new(size);

        svg_builder.render_rectangle(&aabb, &options);
        render_character(
            &mut svg_builder,
            &state.config,
            &aabb,
            character.appearance(),
        );
        let svg = svg_builder.finish();
        RawSvg(svg.export())
    })
}

#[get("/appearance/<id>/edit")]
fn edit_appearance(data: &State<EditorData>, id: usize) -> Option<Template> {
    let data = data.data.lock().expect("lock shared data");
    data.get(CharacterId::new(id)).map(|character| {
        Template::render(
            "appearance_edit",
            context! {
                id: id,
                name: character.name(),
            },
        )
    })
}

fn show_character_template(id: usize, character: &Character) -> Template {
    Template::render(
        "character",
        context! {
            id: id,
            name: character.name(),
            gender: format!("{:?}", character.gender()),
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
            gender: format!("{:?}", character.gender()),
        },
    )
}

#[rocket::main]
async fn main() -> Result<()> {
    if let Err(e) = rocket::build()
        .manage(EditorData {
            config: create_config(),
            data: Mutex::new(init()),
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
