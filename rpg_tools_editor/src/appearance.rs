use rpg_tools_core::model::character::appearance::skin::SkinColor;
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;
use rpg_tools_rendering::math::aabb2d::AABB;
use rpg_tools_rendering::renderer::svg::SvgBuilder;
use rpg_tools_rendering::renderer::Renderer;
use rpg_tools_rendering::rendering::character::{calculate_character_size, render_character};
use rpg_tools_rendering::rendering::config::example::create_border_options;
use rpg_tools_rendering::rendering::config::RenderConfig;
use serde::Serialize;

#[derive(FromForm, Debug)]
pub struct AppearanceUpdate<'r> {
    body_type: &'r str,
    height: u32,
}

impl<'r> AppearanceUpdate<'r> {
    pub fn apply(&self, appearance: &Appearance) -> Appearance {
        match appearance {
            Appearance::HeadOnly { head, .. } => {
                Appearance::head(*head, Length::from_millimetre(self.height))
            }
            Appearance::Humanoid { body, head, .. } => {
                Appearance::humanoid(*body, *head, Length::from_millimetre(self.height))
            }
        }
    }
}

#[derive(Responder)]
#[response(status = 200, content_type = "image/svg+xml")]
pub struct RawSvg(String);

pub fn render_to_svg(config: &RenderConfig, appearance: &Appearance) -> RawSvg {
    let size = calculate_character_size(config, appearance);
    let aabb = AABB::with_size(size);
    let options = create_border_options();
    let mut svg_builder = SvgBuilder::new(size);

    svg_builder.render_rectangle(&aabb, &options);
    render_character(&mut svg_builder, config, &aabb, appearance);
    let svg = svg_builder.finish();
    RawSvg(svg.export())
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct AppearanceOptions {
    body_type: Vec<String>,
    colors: Vec<String>,
    colors_skin: Vec<String>,
}

impl AppearanceOptions {
    pub fn new() -> Self {
        Self {
            body_type: vec!["HeadOnly".to_string(), "Humanoid".to_string()],
            colors: Color::get_all().iter().map(|c| c.to_string()).collect(),
            colors_skin: SkinColor::get_all().iter().map(|c| c.to_string()).collect(),
        }
    }
}
