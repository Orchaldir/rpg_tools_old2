extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use rpg_tools_core::model::color::Color;
use rpg_tools_rendering::renderer::color::WebColor;
use rpg_tools_rendering::renderer::RenderOptions;
use rpg_tools_rendering::rendering::config::head::HeadConfig;

pub fn create_border_options() -> RenderOptions {
    RenderOptions::new(
        WebColor::from_color(Color::White),
        WebColor::from_color(Color::Black),
        5,
    )
}

pub fn create_head_config() -> HeadConfig {
    HeadConfig {
        width_wide: 0.9,
        width_narrow: 0.66,
        width_square: 0.6,
        width_round: 0.4,
        width_sharp: 0.2,
        y_forehead: 0.25,
        y_mouth: 0.75,
    }
}
