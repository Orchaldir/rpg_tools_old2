use crate::math::aabb2d::AABB;
use crate::renderer::Renderer;
use crate::rendering::config::RenderConfig;
use crate::rendering::hair::short::{
    render_buzz_cut_realistic, render_flat_top_realistic, render_middle_part_realistic,
    render_side_part_realistic,
};
use rpg_tools_core::model::character::appearance::hair::{Hair, ShortHair};
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape};

pub mod hairline;
pub mod short;

pub fn render_hair(renderer: &mut dyn Renderer, config: &RenderConfig, aabb: &AABB, head: &Head) {
    match head.shape {
        HeadShape::Geometric(_) => {}
        HeadShape::Realistic(realistic) => match head.hair {
            Hair::None => {}
            Hair::Short {
                style,
                hairline,
                color,
            } => match style {
                ShortHair::BuzzCut => {
                    render_buzz_cut_realistic(renderer, config, aabb, realistic, hairline, color)
                }
                ShortHair::FlatTop(size) => render_flat_top_realistic(
                    renderer, config, aabb, realistic, hairline, size, color,
                ),
                ShortHair::MiddlePart => {
                    render_middle_part_realistic(renderer, config, aabb, realistic, hairline, color)
                }
                ShortHair::SidePart(side) => {
                    render_side_part_realistic(renderer, config, aabb, realistic, side, color)
                }
            },
        },
    }
}
