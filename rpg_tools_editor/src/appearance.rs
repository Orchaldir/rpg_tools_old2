use crate::parser::UrlParser;
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_rendering::math::aabb2d::AABB;
use rpg_tools_rendering::renderer::svg::SvgBuilder;
use rpg_tools_rendering::renderer::Renderer;
use rpg_tools_rendering::rendering::character::{
    calculate_character_size, render_character_from_back, render_character_from_front,
};
use rpg_tools_rendering::rendering::config::example::create_border_options;
use rpg_tools_rendering::rendering::config::RenderConfig;
use url_encoded_data::UrlEncodedData;

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
