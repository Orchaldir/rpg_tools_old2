use rpg_tools_core::model::character::appearance::body::{Body, BodyShape};
use rpg_tools_core::model::character::appearance::ear::shape::EarShape;
use rpg_tools_core::model::character::appearance::ear::Ears;
use rpg_tools_core::model::character::appearance::eye::pupil::PupilShape;
use rpg_tools_core::model::character::appearance::eye::shape::EyeShape;
use rpg_tools_core::model::character::appearance::eye::{Eye, Eyes};
use rpg_tools_core::model::character::appearance::hair::hairline::Hairline;
use rpg_tools_core::model::character::appearance::hair::{Hair, ShortHair};
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape};
use rpg_tools_core::model::character::appearance::mouth::{Mouth, SpecialTeeth, TeethColor};
use rpg_tools_core::model::character::appearance::skin::{Skin, SkinColor};
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;
use rpg_tools_core::model::size::Size;
use rpg_tools_core::model::width::Width;
use rpg_tools_rendering::math::aabb2d::AABB;
use rpg_tools_rendering::renderer::svg::SvgBuilder;
use rpg_tools_rendering::renderer::Renderer;
use rpg_tools_rendering::rendering::character::{calculate_character_size, render_character};
use rpg_tools_rendering::rendering::config::example::create_border_options;
use rpg_tools_rendering::rendering::config::RenderConfig;
use serde::Serialize;
use std::fmt;
use url_encoded_data::UrlEncodedData;
use Appearance::{HeadOnly, Humanoid};

pub fn apply_update_to_appearance(appearance: &Appearance, update: &str) -> Appearance {
    let data = UrlEncodedData::parse_str(update);

    if let Some(height) = data.get_first("appearance.height.millimetre") {
        if let Ok(height) = height.parse::<u32>().map(Length::from_millimetre) {
            return match get_type(&data, "appearance.type") {
                "HeadOnly" => match appearance {
                    HeadOnly { .. } => Appearance::head(update_head(&data), height),
                    Humanoid { .. } => Appearance::head(update_head(&data), height),
                },
                "Humanoid" => match appearance {
                    HeadOnly { .. } => {
                        Appearance::humanoid(Body::default(), update_head(&data), height)
                    }
                    Humanoid { .. } => {
                        Appearance::humanoid(update_body(&data), update_head(&data), height)
                    }
                },
                _ => Appearance::default(),
            };
        }
    }

    Appearance::default()
}

fn update_body(data: &UrlEncodedData) -> Body {
    let width: Width = get_enum(data, "appearance.body.width");
    let shape: BodyShape = get_enum(data, "appearance.body.shape");

    Body {
        shape,
        width,
        skin: update_skin("appearance.body", data),
    }
}

fn update_head(data: &UrlEncodedData) -> Head {
    let shape: HeadShape = get_enum(data, "appearance.head.shape");

    Head {
        shape,
        ears: update_ears(data),
        eyes: update_eyes(data),
        hair: update_hair(data),
        mouth: update_mouth(data),
        skin: update_skin("appearance.head", data),
    }
}

fn update_ears(data: &UrlEncodedData) -> Ears {
    match get_type(data, "appearance.head.ears.type") {
        "Normal" => {
            let shape = get_enum(data, "appearance.head.ears.shape");
            let size = get_enum(data, "appearance.head.ears.size");

            Ears::Normal { shape, size }
        }
        _ => Ears::None,
    }
}

fn update_eyes(data: &UrlEncodedData) -> Eyes {
    match get_type(data, "appearance.head.eyes.type") {
        "One" => Eyes::One {
            eye: update_eye(data),
        },
        "Two" => {
            let distance = get_enum(data, "appearance.head.eyes.distance");

            Eyes::Two {
                eye: update_eye(data),
                distance,
            }
        }
        _ => Eyes::None,
    }
}

fn update_eye(data: &UrlEncodedData) -> Eye {
    let eye_shape = get_enum(data, "appearance.head.eyes.eye.shape");

    match get_type(data, "appearance.head.eyes.eye.type") {
        "Simple" => {
            let color = get_enum(data, "appearance.head.eyes.eye.color");

            Eye::Simple { eye_shape, color }
        }
        "Normal" => {
            let pupil_shape = get_enum(data, "appearance.head.eyes.eye.pupil_shape");
            let pupil_color = get_enum(data, "appearance.head.eyes.eye.pupil_color");
            let background_color = data
                .get_first("appearance.head.eyes.eye.background_color")
                .unwrap_or("White")
                .into();

            Eye::Normal {
                eye_shape,
                pupil_shape,
                pupil_color,
                background_color,
            }
        }
        _ => Eye::default(),
    }
}

fn update_hair(data: &UrlEncodedData) -> Hair {
    match get_type(data, "appearance.head.hair.type") {
        "Short" => {
            let color = get_enum(data, "appearance.head.hair.color");

            Hair::Short {
                style: update_short_hair(data),
                hairline: get_hairline(data),
                color,
            }
        }
        _ => Hair::None,
    }
}

fn update_short_hair(data: &UrlEncodedData) -> ShortHair {
    match get_type(data, "appearance.head.hair.style.type") {
        "FlatTop" => {
            let size = get_enum(data, "appearance.head.hair.style.c");
            ShortHair::FlatTop(size)
        }
        "MiddlePart" => ShortHair::MiddlePart,
        "SidePart" => {
            let side = get_enum(data, "appearance.head.hair.style.c");
            ShortHair::SidePart(side)
        }
        _ => ShortHair::BuzzCut,
    }
}

fn get_hairline(data: &UrlEncodedData) -> Hairline {
    let size = get_enum(data, "appearance.head.hair.hairline.c");

    match get_type(data, "appearance.head.hair.hairline.type") {
        "Straight" => Hairline::Straight(size),
        "Triangle" => Hairline::Triangle(size),
        "WidowsPeak" => Hairline::WidowsPeak(size),
        _ => Hairline::Round(size),
    }
}

fn update_mouth(data: &UrlEncodedData) -> Mouth {
    match get_type(data, "appearance.head.mouth.type") {
        "Circle" => {
            let (size, teeth_color) = parse_common_mouth(data);

            Mouth::Circle { size, teeth_color }
        }
        "Normal" => {
            let (width, teeth_color) = parse_common_mouth(data);

            Mouth::Normal {
                width,
                teeth: parse_special_teeth(data),
                teeth_color,
            }
        }
        _ => Mouth::None,
    }
}

fn parse_common_mouth(data: &UrlEncodedData) -> (Size, TeethColor) {
    let size = get_enum(data, "appearance.head.mouth.size");
    let color = get_enum(data, "appearance.head.mouth.teeth_color");

    (size, color)
}

fn parse_special_teeth(data: &UrlEncodedData) -> SpecialTeeth {
    let size = get_enum(data, "appearance.head.mouth.teeth.c");

    match get_type(data, "appearance.head.mouth.teeth.type") {
        "LowerFangs" => SpecialTeeth::LowerFangs(size),
        "UpperFangs" => SpecialTeeth::UpperFangs(size),
        _ => SpecialTeeth::None,
    }
}

fn update_skin(path: &str, data: &UrlEncodedData) -> Skin {
    let color = data.get_first(&format!("{}.skin.c", path)).unwrap_or("");

    match get_type(data, &format!("{}.skin.type", path)) {
        "Scales" => {
            let color = Color::from(color);
            Skin::Scales(color)
        }
        "ExoticSkin" => {
            let color = Color::from(color);
            Skin::ExoticSkin(color)
        }
        _ => {
            let color = SkinColor::from(color);
            Skin::Skin(color)
        }
    }
}

fn get_enum<'a, T: From<&'a str>>(data: &'a UrlEncodedData, path: &str) -> T {
    data.get_first(path).unwrap_or("").into()
}

fn get_type<'a>(data: &'a UrlEncodedData, path: &str) -> &'a str {
    data.get_first(path).unwrap_or("")
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
    body_types: Vec<String>,
    body_shapes: Vec<String>,
    colors: Vec<String>,
    colors_skin: Vec<String>,
    ears: Vec<String>,
    ear_shapes: Vec<String>,
    eyes: Vec<String>,
    eye_shapes: Vec<String>,
    eye: Vec<String>,
    head_shapes: Vec<String>,
    pupil_shapes: Vec<String>,
    sizes: Vec<String>,
    skin_types: Vec<String>,
    widths: Vec<String>,
}

impl AppearanceOptions {
    pub fn new() -> Self {
        Self {
            body_types: vec!["HeadOnly".to_string(), "Humanoid".to_string()],
            body_shapes: convert(BodyShape::get_all()),
            colors: convert(Color::get_all()),
            colors_skin: convert(SkinColor::get_all()),
            ears: vec!["None".to_string(), "Normal".to_string()],
            ear_shapes: convert(EarShape::get_all()),
            eyes: vec!["None".to_string(), "One".to_string(), "Two".to_string()],
            eye_shapes: convert(EyeShape::get_all()),
            eye: vec!["Normal".to_string(), "Simple".to_string()],
            head_shapes: convert(HeadShape::get_all()),
            pupil_shapes: convert(PupilShape::get_all()),
            sizes: convert(Size::get_all()),
            skin_types: vec![
                "Scales".to_string(),
                "Skin".to_string(),
                "ExoticSkin".to_string(),
            ],
            widths: convert(Width::get_all()),
        }
    }
}

impl Default for AppearanceOptions {
    fn default() -> Self {
        Self::new()
    }
}

fn convert(options: Vec<impl fmt::Display>) -> Vec<String> {
    options.iter().map(|c| c.to_string()).collect()
}
