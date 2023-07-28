use rpg_tools_core::model::character::appearance::body::{Body, BodyShape};
use rpg_tools_core::model::character::appearance::ear::shape::EarShape;
use rpg_tools_core::model::character::appearance::ear::Ears;
use rpg_tools_core::model::character::appearance::eye::pupil::PupilShape;
use rpg_tools_core::model::character::appearance::eye::shape::EyeShape;
use rpg_tools_core::model::character::appearance::eye::{Eye, Eyes};
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

    if let Some(t) = data.get_first("appearance.type") {
        if let Some(height) = data.get_first("appearance.height.millimetre") {
            if let Ok(height) = height.parse::<u32>().map(Length::from_millimetre) {
                match t {
                    "HeadOnly" => {
                        return match appearance {
                            HeadOnly { head, .. } => {
                                Appearance::head(update_head(head, &data), height)
                            }
                            Humanoid { head, .. } => {
                                Appearance::head(update_head(head, &data), height)
                            }
                        }
                    }
                    "Humanoid" => {
                        return match appearance {
                            HeadOnly { head, .. } => Appearance::humanoid(
                                Body::default(),
                                update_head(head, &data),
                                height,
                            ),
                            Humanoid { head, .. } => Appearance::humanoid(
                                update_body(&data),
                                update_head(head, &data),
                                height,
                            ),
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    Appearance::default()
}

fn update_body(data: &UrlEncodedData) -> Body {
    if let Some(width) = data.get_first("appearance.body.width") {
        if let Some(shape) = data.get_first("appearance.body.shape") {
            let width: Width = width.into();
            let shape: BodyShape = shape.into();

            return Body {
                shape,
                width,
                skin: update_skin("appearance.body", data),
            };
        }
    }

    Body::default()
}

fn update_head(head: &Head, data: &UrlEncodedData) -> Head {
    let shape: HeadShape = data.get_first("appearance.head.shape").unwrap_or("").into();

    Head {
        shape,
        ears: update_ears(data),
        eyes: update_eyes(data),
        mouth: update_mouth(data),
        skin: update_skin("appearance.head", data),
        ..*head
    }
}

fn update_ears(data: &UrlEncodedData) -> Ears {
    if let Some(t) = data.get_first("appearance.head.ears.type") {
        return match t {
            "Normal" => {
                let shape = data
                    .get_first("appearance.head.ears.shape")
                    .unwrap_or("")
                    .into();
                let size = data
                    .get_first("appearance.head.ears.size")
                    .unwrap_or("")
                    .into();

                Ears::Normal { shape, size }
            }
            _ => Ears::None,
        };
    }

    Ears::None
}

fn update_eyes(data: &UrlEncodedData) -> Eyes {
    if let Some(t) = data.get_first("appearance.head.eyes.type") {
        return match t {
            "One" => Eyes::One {
                eye: update_eye(data),
            },
            "Two" => {
                let distance = data
                    .get_first("appearance.head.eyes.distance")
                    .unwrap_or("")
                    .into();

                Eyes::Two {
                    eye: update_eye(data),
                    distance,
                }
            }
            _ => Eyes::None,
        };
    }

    Eyes::None
}

fn update_eye(data: &UrlEncodedData) -> Eye {
    if let Some(t) = data.get_first("appearance.head.eyes.eye.type") {
        let eye_shape = data
            .get_first("appearance.head.eyes.eye.shape")
            .unwrap_or("")
            .into();

        return match t {
            "Simple" => {
                let color = data
                    .get_first("appearance.head.eyes.eye.color")
                    .unwrap_or("")
                    .into();

                Eye::Simple { eye_shape, color }
            }
            "Normal" => {
                let pupil_shape = data
                    .get_first("appearance.head.eyes.eye.pupil_shape")
                    .unwrap_or("")
                    .into();
                let pupil_color = data
                    .get_first("appearance.head.eyes.eye.pupil_color")
                    .unwrap_or("")
                    .into();
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
        };
    }

    Eye::default()
}

fn update_mouth(data: &UrlEncodedData) -> Mouth {
    if let Some(t) = data.get_first("appearance.head.mouth.type") {
        return match t {
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
        };
    }

    Mouth::None
}

fn parse_common_mouth(data: &UrlEncodedData) -> (Size, TeethColor) {
    let size = data
        .get_first("appearance.head.mouth.size")
        .unwrap_or("")
        .into();
    let color = data
        .get_first("appearance.head.mouth.teeth_color")
        .unwrap_or("")
        .into();

    (size, color)
}

fn parse_special_teeth(data: &UrlEncodedData) -> SpecialTeeth {
    let size = data
        .get_first("appearance.head.mouth.teeth.c")
        .unwrap_or("")
        .into();

    if let Some(t) = data.get_first("appearance.head.mouth.teeth.type") {
        return match t {
            "LowerFangs" => SpecialTeeth::LowerFangs(size),
            "UpperFangs" => SpecialTeeth::UpperFangs(size),
            _ => SpecialTeeth::None,
        };
    }

    SpecialTeeth::None
}

fn update_skin(path: &str, data: &UrlEncodedData) -> Skin {
    if let Some(t) = data.get_first(&format!("{}.skin.type", path)) {
        if let Some(c) = data.get_first(&format!("{}.skin.c", path)) {
            return match t {
                "Scales" => {
                    let color = Color::from(c);
                    Skin::Scales(color)
                }
                "ExoticSkin" => {
                    let color = Color::from(c);
                    Skin::ExoticSkin(color)
                }
                _ => {
                    let color = SkinColor::from(c);
                    Skin::Skin(color)
                }
            };
        }
    }

    Skin::default()
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
