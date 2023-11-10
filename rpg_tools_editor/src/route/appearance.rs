use crate::parser::UrlParser;
use crate::route::character::save_and_show_character;
use crate::EditorData;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::character::{Character, CharacterId};
use rpg_tools_rendering::math::aabb2d::AABB;
use rpg_tools_rendering::renderer::svg::SvgBuilder;
use rpg_tools_rendering::renderer::Renderer;
use rpg_tools_rendering::rendering::character::{
    calculate_character_size, render_character_from_back, render_character_from_front,
};
use rpg_tools_rendering::rendering::config::example::create_border_options;
use rpg_tools_rendering::rendering::config::RenderConfig;
use url_encoded_data::UrlEncodedData;

#[get("/appearance/render/<id>/front.svg")]
pub fn get_appearance_front(state: &State<EditorData>, id: usize) -> Option<RawSvg> {
    let data = state.data.lock().expect("lock shared data");
    data.character_manager
        .get(CharacterId::new(id))
        .map(|character| render_to_svg(&state.config, character.appearance(), true))
}

#[get("/appearance/render/<id>/back.svg")]
pub fn get_appearance_back(state: &State<EditorData>, id: usize) -> Option<RawSvg> {
    let data = state.data.lock().expect("lock shared data");
    data.character_manager
        .get(CharacterId::new(id))
        .map(|character| render_to_svg(&state.config, character.appearance(), false))
}

#[get("/appearance/preview/front.svg")]
pub fn get_preview_front(state: &State<EditorData>) -> RawSvg {
    let preview = state.preview.lock().expect("lock shared preview");
    render_to_svg(&state.config, &preview, true)
}

#[get("/appearance/preview/back.svg")]
pub fn get_preview_back(state: &State<EditorData>) -> RawSvg {
    let preview = state.preview.lock().expect("lock shared preview");
    render_to_svg(&state.config, &preview, false)
}

#[get("/appearance/edit/<id>")]
pub fn edit_appearance(state: &State<EditorData>, id: usize) -> Option<Template> {
    let data = state.data.lock().expect("lock shared data");
    let mut preview = state.preview.lock().expect("lock shared preview");

    data.character_manager
        .get(CharacterId::new(id))
        .map(|character| {
            *preview = *character.appearance();

            edit_appearance_template(character, &preview)
        })
}

#[post("/appearance/update/<id>", data = "<update>")]
pub fn update_appearance(data: &State<EditorData>, id: usize, update: String) -> Option<Template> {
    let mut data = data.data.lock().expect("lock shared data");

    println!("Update appearance of character {} with {:?}", id, update);

    data.character_manager
        .get_mut(CharacterId::new(id))
        .map(|character| {
            character.set_appearance(apply_update_to_appearance(&update));
            character
        });

    save_and_show_character(&data, id)
}

#[post("/appearance/preview/<id>", data = "<update>")]
pub fn update_appearance_preview(
    state: &State<EditorData>,
    id: usize,
    update: String,
) -> Option<Template> {
    let data = state.data.lock().expect("lock shared data");
    let mut preview = state.preview.lock().expect("lock shared preview");

    println!("Preview appearance of character {} with {:?}", id, update);

    data.character_manager
        .get(CharacterId::new(id))
        .map(|character| {
            *preview = apply_update_to_appearance(&update);

            edit_appearance_template(character, &preview)
        })
}

fn edit_appearance_template(character: &Character, appearance: &Appearance) -> Template {
    Template::render(
        "appearance_edit",
        context! {
            id: character.id().id(),
            name: character.name(),
            appearance: appearance,
        },
    )
}

pub fn apply_update_to_appearance(update: &str) -> Appearance {
    let data = UrlEncodedData::parse_str(update);
    let parser = UrlParser::new(data);

    Appearance::parse(&parser, "appearance", "")
}

#[derive(Responder)]
#[response(status = 200, content_type = "image/svg+xml")]
pub struct RawSvg(String);

pub fn render_to_svg(config: &RenderConfig, appearance: &Appearance, front: bool) -> RawSvg {
    let size = calculate_character_size(config, appearance);
    let aabb = AABB::with_size(size);
    let options = create_border_options();
    let mut svg_builder = SvgBuilder::new(size);

    svg_builder.render_rectangle(&aabb, &options);

    if front {
        render_character_from_front(&mut svg_builder, config, &aabb, appearance);
    } else {
        render_character_from_back(&mut svg_builder, config, &aabb, appearance);
    }

    let svg = svg_builder.finish();
    RawSvg(svg.export())
}
