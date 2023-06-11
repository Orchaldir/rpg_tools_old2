extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use rpg_tools_core::model::character::appearance::eye::{Eye, EyeShape, Eyes, PupilShape};
use rpg_tools_core::model::character::appearance::head::RealisticHeadShape::*;
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape};
use rpg_tools_core::model::character::appearance::skin::Skin;
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;
use rpg_tools_rendering::math::aabb2d::AABB;
use rpg_tools_rendering::renderer::color::WebColor;
use rpg_tools_rendering::renderer::svg::SvgBuilder;
use rpg_tools_rendering::renderer::{RenderOptions, Renderer};
use rpg_tools_rendering::rendering::body::BodyRenderer;
use rpg_tools_rendering::rendering::character::{calculate_character_size, render_character};
use rpg_tools_rendering::rendering::RenderConfig;

fn main() {
    let config = RenderConfig {
        border: 500,
        line_color: WebColor::from_color(Color::Black),
        line_width: 50,
        body_renderer: BodyRenderer {},
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
                eyes: Eyes::One(Eye::Normal {
                    eye_shape: EyeShape::Round,
                    pupil_shape: PupilShape::VerticalSlit,
                    pupil_color: Color::Black,
                    iris_color: Color::Red,
                    background_color: Color::White,
                }),
                shape: HeadShape::Realistic(*realistic),
                skin: Skin::Scales(Color::Red),
            },
            Length::from_metre(1.0),
        );
        let size = calculate_character_size(&config, &appearance);
        let aabb = AABB::with_size(size);
        let mut svg_builder = SvgBuilder::new(size);

        svg_builder.render_rectangle(&aabb, &options);
        render_character(&mut svg_builder, &config, &aabb, &appearance);
        let svg = svg_builder.finish();
        svg.save(&format!("{}-{:?}.svg", i, realistic)).unwrap();
    }
}
