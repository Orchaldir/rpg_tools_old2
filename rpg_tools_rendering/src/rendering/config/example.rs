extern crate rpg_tools_core;

use crate::renderer::color::WebColor;
use crate::renderer::RenderOptions;
use crate::rendering::config::ear::EarConfig;
use crate::rendering::config::eye::EyeConfig;
use crate::rendering::config::hair::hairline::HairlineConfig;
use crate::rendering::config::hair::short::ShortHairConfig;
use crate::rendering::config::hair::HairConfig;
use crate::rendering::config::head::HeadConfig;
use crate::rendering::config::mouth::{CircularMouthConfig, MouthConfig};
use crate::rendering::config::size::SizeConfig;
use crate::rendering::config::RenderConfig;
use rpg_tools_core::model::color::Color;

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
                y: SizeConfig {
                    low: 0.25,
                    medium: 0.2,
                    high: 0.15,
                },
            },
            short: ShortHairConfig {
                side_part_offset: 0.3,
                y_flat_top: SizeConfig {
                    low: 0.0,
                    medium: -0.1,
                    high: -0.2,
                },
                y_middle_part: SizeConfig {
                    low: 0.35,
                    medium: 0.3,
                    high: 0.25,
                },
                inner_width: 0.9,
                hairline_width: 0.6,
            },
        },
        head: create_head_config(),
        ear: create_ear_config(),
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

pub fn create_ear_config() -> EarConfig {
    EarConfig {
        normal_offset: 0.02,
        normal_top_x: 0.04,
        normal_width: 0.08,
        pointed_length: SizeConfig {
            low: 0.1,
            medium: 0.2,
            high: 0.3,
        },
        round_factor: 0.1,
    }
}

pub fn create_eye_config() -> EyeConfig {
    EyeConfig {
        radius: 0.125,
        half_height_almond: 0.7,
        half_height_ellipse: 0.75,
        distance_between_eyes: SizeConfig {
            low: 0.35,
            medium: 0.4,
            high: 0.45,
        },
        circle_radius: 0.5,
        slit_width: 0.2,
    }
}

pub fn create_mouth_config() -> MouthConfig {
    MouthConfig {
        mouth_width: SizeConfig {
            low: 0.4,
            medium: 0.5,
            high: 0.6,
        },
        distance_between_fangs: 0.6,
        fang_height: SizeConfig {
            low: 0.08,
            medium: 0.1,
            high: 0.16,
        },
        circular: CircularMouthConfig {
            radius: SizeConfig {
                low: 0.2,
                medium: 0.25,
                high: 0.3,
            },
            fang_height: 0.5,
        },
    }
}