use crate::math::aabb2d::AABB;
use crate::math::polygon2d::Polygon2d;
use crate::renderer::Renderer;
use crate::rendering::config::RenderConfig;
use crate::rendering::hair::short::{
    get_flat_top_realistic, get_middle_part_realistic, get_side_part_realistic,
    render_buzz_cut_realistic,
};
use rpg_tools_core::model::character::appearance::hair::{Hair, HairColor, ShortHair};
use rpg_tools_core::model::character::appearance::head::Head;

pub mod hairline;
pub mod short;

pub fn render_hair(renderer: &mut dyn Renderer, config: &RenderConfig, aabb: &AABB, head: &Head) {
    match head.hair {
        Hair::None => {}
        Hair::Short {
            style,
            hairline,
            color,
        } => match style {
            ShortHair::BuzzCut => {
                render_buzz_cut_realistic(renderer, config, aabb, head.shape, hairline, color)
            }
            ShortHair::FlatTop(size) => {
                let polygon = get_flat_top_realistic(config, aabb, head.shape, hairline, size);
                render_hair_polygon(renderer, config, &polygon, color);
            }
            ShortHair::MiddlePart => {
                let polygon = get_middle_part_realistic(config, aabb, head.shape, hairline);
                render_hair_polygon(renderer, config, &polygon, color);
            }
            ShortHair::SidePart(side) => {
                let polygon = get_side_part_realistic(config, aabb, head.shape, side);
                render_hair_polygon(renderer, config, &polygon, color);
            }
        },
    }
}

pub fn render_hair_polygon(
    renderer: &mut dyn Renderer,
    config: &RenderConfig,
    polygon: &Polygon2d,
    color: HairColor,
) {
    let options = config.get_hair_options(color);
    renderer.render_polygon(polygon, &options);
}
