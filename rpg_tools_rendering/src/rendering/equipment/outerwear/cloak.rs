use crate::math::aabb2d::AABB;
use crate::math::line2d::Line2d;
use crate::renderer::Renderer;
use crate::rendering::config::RenderConfig;
use crate::rendering::equipment::pants::interpolate_pants_y;
use crate::rendering::equipment::part::button::render_buttons;
use crate::rendering::equipment::part::neckline::get_neckline_bottom_y;
use crate::rendering::equipment::part::sleeve::render_sleeves;
use crate::rendering::equipment::shirt::create_shirt;
use rpg_tools_core::model::character::appearance::body::Body;
use rpg_tools_core::model::equipment::appearance::outerwear::cloak::Cloak;
use rpg_tools_core::model::equipment::appearance::outerwear::coat::{
    ClosingOption, Coat, OuterwearLength,
};

pub fn render_cloak_before_body(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    cloak: &Cloak,
    from_front: bool,
) {
}

pub fn render_cloak_behind_body(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    cloak: &Cloak,
    from_front: bool,
) {
}
