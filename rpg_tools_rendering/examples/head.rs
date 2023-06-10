extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use rpg_tools_core::model::character::appearance::head::RealisticHeadShape::*;
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape};
use rpg_tools_core::model::character::appearance::skin::{Skin, SkinColor};
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;
use rpg_tools_rendering::math::aabb2d::AABB;
use rpg_tools_rendering::renderer::color::WebColor;
use rpg_tools_rendering::renderer::svg::SvgBuilder;
use rpg_tools_rendering::renderer::{RenderOptions, Renderer};
use rpg_tools_rendering::rendering::body::BodyRenderer;
use rpg_tools_rendering::rendering::character::CharacterRenderer;
use rpg_tools_rendering::rendering::head::HeadRenderer;
use rpg_tools_rendering::rendering::RenderConfig;

fn main() {
    let config = RenderConfig {
        line_color: WebColor::from_color(Color::Black),
        line_width: 50,
    };
    let character_renderer = CharacterRenderer {
        border: 500,
        body_renderer: BodyRenderer {},
        head_renderer: HeadRenderer {},
    };
    let options = RenderOptions::new(
        WebColor::from_color(Color::White),
        WebColor::from_color(Color::Black),
        10,
    );

    for (i, realistic) in [Oval, Rectangle, Round, Square, TriangleDown, TriangleUp]
        .iter()
        .enumerate()
    {
        let appearance = Appearance::head(
            Head {
                shape: HeadShape::Realistic(*realistic),
                skin: Skin::Skin(SkinColor::Light),
            },
            Length::from_metre(1.0),
        );
        let size = character_renderer.calculate_size(&appearance);
        let aabb = AABB::with_size(size);
        let mut svg_builder = SvgBuilder::new(size);

        svg_builder.render_rectangle(&aabb, &options);
        character_renderer.render(&mut svg_builder, &config, &aabb, &appearance);
        let svg = svg_builder.finish();
        svg.save(&format!("{}-{:?}.svg", i, realistic)).unwrap();
    }
}
