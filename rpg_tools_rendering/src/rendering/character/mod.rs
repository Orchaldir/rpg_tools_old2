use crate::math::aabb2d::AABB;
use crate::math::size2d::Size2d;
use crate::renderer::Renderer;
use crate::rendering::character::body::{calculate_head_aabb, render_body, render_hands};
use crate::rendering::character::hair::{
    render_hair_back, render_hair_before_head_from_front, render_hair_behind_head_from_front,
};
use crate::rendering::config::RenderConfig;
use crate::rendering::equipment::{
    render_clothing, render_clothing_behind_body, render_clothing_over_hands,
};
use ear::render_ears;
use eye::render_eyes;
use head::render_head_shape;
use mouth::render_mouth;
use rpg_tools_core::model::appearance::length::Length;
use rpg_tools_core::model::character::appearance::head::Head;
use rpg_tools_core::model::character::appearance::Appearance;

pub mod beard;
pub mod body;
pub mod ear;
pub mod eye;
pub mod hair;
pub mod head;
pub mod mouth;

pub fn calculate_character_size(config: &RenderConfig, appearance: &Appearance) -> Size2d {
    calculate_size(config, appearance.calculate_height())
}

pub fn calculate_size(config: &RenderConfig, height: Length) -> Size2d {
    Size2d::square(height.to_millimetre() + config.border * 2)
}

pub fn render_character_from_front(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    appearance: &Appearance,
) {
    let inner = aabb.shrink(config.border);

    match appearance {
        Appearance::HeadOnly { head, .. } => {
            render_head_before_body_from_front(renderer, config, head, &inner);
        }
        Appearance::Humanoid { body, head, .. } => {
            let head_aabb = calculate_head_aabb(config, &inner);
            render_head_behind_body_from_front(renderer, config, head, &head_aabb);
            render_clothing_behind_body(renderer, config, &inner, body, true);
            render_body(renderer, config, &inner, body);
            render_clothing(renderer, config, &inner, body, true);
            render_hands(renderer, config, &inner, body);
            render_head_before_body_from_front(renderer, config, head, &head_aabb);
        }
    }
}

pub fn render_character_from_back(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    appearance: &Appearance,
) {
    let inner = aabb.shrink(config.border);

    match appearance.mirror() {
        Appearance::HeadOnly { head, .. } => {
            render_head_from_back(renderer, config, &head, &inner);
        }
        Appearance::Humanoid { body, head, .. } => {
            render_clothing_behind_body(renderer, config, &inner, &body, false);
            render_body(renderer, config, &inner, &body);
            render_clothing(renderer, config, &inner, &body, false);
            render_hands(renderer, config, &inner, &body);
            render_clothing_over_hands(renderer, config, &inner, &body, false);
            let head_aabb = calculate_head_aabb(config, &inner);
            render_head_from_back(renderer, config, &head, &head_aabb);
        }
    }
}

fn render_head_before_body_from_front(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    head: &Head,
    aabb: &AABB,
) {
    render_ears(renderer, config, aabb, head);
    render_head_shape(renderer, config, aabb, head);
    render_hair_before_head_from_front(renderer, config, aabb, head);
    render_mouth(renderer, config, aabb, head);
    render_eyes(renderer, config, aabb, head);
}

fn render_head_behind_body_from_front(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    head: &Head,
    aabb: &AABB,
) {
    render_hair_behind_head_from_front(renderer, config, aabb, head);
}

fn render_head_from_back(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    head: &Head,
    aabb: &AABB,
) {
    render_ears(renderer, config, aabb, head);
    render_head_shape(renderer, config, aabb, head);
    render_hair_back(renderer, config, aabb, head);
}
