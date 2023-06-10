extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use rpg_tools_core::model::character::appearance::head::{GeometricHeadShape, Head, HeadShape};
use rpg_tools_core::model::character::appearance::skin::Skin;
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;
use rpg_tools_rendering::math::aabb2d::AABB;
use rpg_tools_rendering::renderer::svg::SvgBuilder;
use rpg_tools_rendering::rendering::body::BodyRenderer;
use rpg_tools_rendering::rendering::character::CharacterRenderer;
use rpg_tools_rendering::rendering::head::HeadRenderer;

fn main() {
    let appearance = Appearance::head(
        Head {
            shape: HeadShape::Geometric(GeometricHeadShape::Square),
            skin: Skin::Scales(Color::Red),
        },
        Length::from_metre(1.0),
    );
    let character_renderer = CharacterRenderer {
        border: 200,
        body_renderer: BodyRenderer {},
        head_renderer: HeadRenderer {},
    };
    let size = character_renderer.calculate_size(&appearance);
    let aabb = AABB::with_size(size);
    let mut svg_builder = SvgBuilder::new(size);

    character_renderer.render(&mut svg_builder, &aabb, &appearance);
    let svg = svg_builder.finish();
    svg.save("beholder.svg").unwrap();
}
