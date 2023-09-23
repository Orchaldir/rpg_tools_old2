use crate::math::aabb2d::AABB;
use crate::math::polygon2d::builder::Polygon2dBuilder;
use crate::renderer::Renderer;
use crate::rendering::body::torso::create_torso;
use crate::rendering::body::{get_left_arm, get_left_arm_short};
use crate::rendering::config::body::torso::TorsoConfig;
use crate::rendering::config::equipment::shirt::ShirtConfig;
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::character::appearance::body::Body;
use rpg_tools_core::model::equipment::appearance::footwear::Footwear;
use rpg_tools_core::model::equipment::appearance::shirt::{Neckline, Shirt, SleeveStyle};

pub fn render_footwear(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    footwear: &Footwear,
    from_front: bool,
) {
    render_sole(renderer, config, aabb, body, footwear);
}

fn render_sole(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    footwear: &Footwear,
) {
}
