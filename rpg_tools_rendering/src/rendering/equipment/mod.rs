use crate::math::aabb2d::AABB;
use crate::renderer::Renderer;
use crate::rendering::config::RenderConfig;
use crate::rendering::equipment::footwear::render_footwear;
use crate::rendering::equipment::pants::render_pants;
use crate::rendering::equipment::shirt::render_shirt;
use rpg_tools_core::model::character::appearance::body::Body;
use rpg_tools_core::model::equipment::appearance::Clothing;

pub mod belt;
pub mod footwear;
pub mod pants;
pub mod shirt;

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
    } = &body.clothing
    {
        render_shirt(renderer, config, aabb, body, shirt, from_front);

        if footwear.style.is_over_pants() {
            let shaft_y = config
                .footwear
                .get_shaft_y(&config.body, body, footwear.style);
            render_pants(renderer, config, aabb, body, pants, shaft_y);
            render_footwear(renderer, config, aabb, body, footwear, from_front);
        } else {
            render_footwear(renderer, config, aabb, body, footwear, from_front);
            render_pants(renderer, config, aabb, body, pants, None);
        }
    }
}
