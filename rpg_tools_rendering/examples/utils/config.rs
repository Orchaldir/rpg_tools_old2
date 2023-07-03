extern crate rpg_tools_core;
extern crate rpg_tools_rendering;

use rpg_tools_core::model::color::Color;
use rpg_tools_rendering::renderer::color::WebColor;
use rpg_tools_rendering::renderer::RenderOptions;
use rpg_tools_rendering::rendering::config::eye::EyeConfig;
use rpg_tools_rendering::rendering::config::hair::hairline::HairlineConfig;
use rpg_tools_rendering::rendering::config::hair::short::ShortHairConfig;
use rpg_tools_rendering::rendering::config::hair::HairConfig;
use rpg_tools_rendering::rendering::config::head::HeadConfig;
use rpg_tools_rendering::rendering::config::mouth::{CircularMouthConfig, MouthConfig};
use rpg_tools_rendering::rendering::config::RenderConfig;

pub fn create_border_options() -> RenderOptions {
    RenderOptions::new(
        WebColor::from_color(Color::White),
        WebColor::from_color(Color::Black),
        5,
    )
}

pub fn create_config() -> RenderConfig {
    RenderConfig {
        border: 500,
        line_color: WebColor::from_color(Color::Black),
        line_width: 30,
        cut_corners_u: 0.25,
        cut_corners_v: 0.25,
        cut_corners_n: 3,
        hair: HairConfig {
            hairline: HairlineConfig {
                width_round: 0.4,
                width_straight: 0.6,
                width_triangle: 0.2,
                width_widows_peak: 0.4,
                height_widows_peak: 0.1,
                y_low: 0.25,
                y_medium: 0.2,
                y_high: 0.15,
            },
            short: ShortHairConfig {
                side_part_offset: 0.3,
                y_flat_top_low: 0.0,
                y_flat_top_medium: -0.1,
                y_flat_top_high: -0.2,
                y_middle_part_low: 0.35,
                y_middle_part_medium: 0.3,
                y_middle_part_high: 0.25,
            },
        },
        head: create_head_config(),
        eye: create_eye_config(),
        mouth: create_mouth_config(),
    }
}

pub fn create_head_config() -> HeadConfig {
    HeadConfig {
        width_wide: 0.9,
        width_narrow: 0.66,
        width_square: 0.6,
        width_round: 0.4,
        width_sharp: 0.2,
        y_forehead: 0.25,
        y_eye: 0.5,
        y_mouth: 0.75,
    }
}

pub fn create_eye_config() -> EyeConfig {
    EyeConfig {
        radius: 0.125,
        half_height_almond: 0.7,
        half_height_ellipse: 0.75,
        low_distance: 0.35,
        medium_distance: 0.4,
        high_distance: 0.45,
        circle_radius: 0.5,
        slit_width: 0.2,
    }
}

pub fn create_mouth_config() -> MouthConfig {
    MouthConfig {
        mouth_width_low: 0.4,
        mouth_width_medium: 0.5,
        mouth_width_high: 0.6,
        distance_between_fangs: 0.6,
        fang_height_low: 0.08,
        fang_height_medium: 0.12,
        fang_height_high: 0.16,
        circular: CircularMouthConfig {
            radius_low: 0.2,
            radius_medium: 0.25,
            radius_high: 0.3,
            fang_height: 0.5,
        },
    }
}
