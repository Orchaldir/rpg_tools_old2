use crate::math::aabb2d::AABB;
use crate::renderer::Renderer;
use crate::rendering::RenderConfig;
use rpg_tools_core::model::character::appearance::body::Body;
use std::ops::Mul;

/// Renders a [`body`](Body).
#[derive(Debug, PartialEq, Eq)]
pub struct BodyRenderer {}

impl BodyRenderer {
    pub fn render(
        &self,
        renderer: &mut dyn Renderer,
        config: &RenderConfig,
        aabb: &AABB,
        body: &Body,
    ) {
        let options = config.get_options(&body.skin);
        let height = aabb.size().height();

        let body_height = 0.5;
        let arms_height = 0.36;
        let legs_height = 0.21;
        let feet_height = 0.07;

        let thin_width = 0.25;
        let average_width = 0.35;
        let wide_width = 0.45;
        let arms_width = 0.1;
        let legs_width = 0.14;
        let feet_width = 0.21;

        let head_size = 0.286;
        let hands_size = 0.14;

        let torso_y = 0.21;

        let torso_start = aabb.get_point(0.5 - average_width / 2.0, torso_y);
        let torso_size = aabb.size().scale(average_width, body_height);
        let torso_aabb = AABB::new(torso_start, torso_size);

        renderer.render_rectangle(&torso_aabb, &options);
    }

    pub fn calculate_head_aabb(&self, aabb: &AABB) -> AABB {
        let head_size = 0.286;

        let head_start = aabb.get_point(0.5 - head_size / 2.0, 0.0);
        let head_size = aabb.size().mul(head_size);
        AABB::new(head_start, head_size)
    }
}
