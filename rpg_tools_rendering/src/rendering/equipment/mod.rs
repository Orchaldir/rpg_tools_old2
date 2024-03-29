use crate::math::aabb2d::AABB;
use crate::renderer::Renderer;
use crate::rendering::config::RenderConfig;
use crate::rendering::equipment::belt::render_belt;
use crate::rendering::equipment::footwear::render_footwear;
use crate::rendering::equipment::outerwear::{
    render_outerwear_before_body, render_outerwear_behind_body, render_outerwear_over_hands,
};
use crate::rendering::equipment::pants::render_pants;
use crate::rendering::equipment::shirt::render_shirt;
use rpg_tools_core::model::character::appearance::body::Body;
use rpg_tools_core::model::equipment::appearance::Clothing;

pub mod belt;
pub mod eyewear;
pub mod footwear;
pub mod outerwear;
pub mod pants;
pub mod part;
pub mod shirt;

pub fn render_clothing_behind_body(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    from_front: bool,
) {
    if let Clothing::Simple { outerwear, .. } = &body.clothing {
        render_outerwear_behind_body(renderer, config, aabb, body, outerwear, from_front);
    }
}

pub fn render_clothing(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    from_front: bool,
) {
    if let Clothing::Simple {
        footwear,
        pants,
        shirt,
        outerwear,
    } = &body.clothing
    {
        render_shirt(renderer, config, aabb, body, shirt, from_front);

        if footwear.style.is_over_pants() {
            let shaft_y = config
                .footwear
                .get_shaft_y(&config.body, body, footwear.style);
            render_pants(renderer, config, aabb, body, pants, shaft_y);
            if let Some(belt) = &pants.belt {
                render_belt(renderer, config, aabb, body, belt, from_front);
            }
            render_footwear(renderer, config, aabb, body, footwear, from_front);
        } else {
            render_footwear(renderer, config, aabb, body, footwear, from_front);
            render_pants(renderer, config, aabb, body, pants, None);
            if let Some(belt) = &pants.belt {
                render_belt(renderer, config, aabb, body, belt, from_front);
            }
        }

        render_outerwear_before_body(renderer, config, aabb, body, outerwear, from_front);
    }
}

pub fn render_clothing_over_hands(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    aabb: &AABB,
    body: &Body,
    from_front: bool,
) {
    if let Clothing::Simple { outerwear, .. } = &body.clothing {
        render_outerwear_over_hands(renderer, config, aabb, body, outerwear, from_front);
    }
}
