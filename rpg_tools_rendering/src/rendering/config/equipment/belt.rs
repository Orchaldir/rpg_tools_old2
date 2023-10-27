use crate::math::aabb2d::AABB;
use crate::math::point2d::Point2d;
use crate::math::size2d::Size2d;
use crate::rendering::config::body::BodyConfig;
use crate::rendering::config::equipment::pants::PantsConfig;
use rpg_tools_core::model::character::appearance::body::Body;

/// The rendering config of the [`belt`](Belt).
#[derive(Debug, PartialEq)]
pub struct BeltConfig {
    pub y_offset: f32,
    pub height: f32,
    pub thickness: f32,
    pub buckle: BuckleConfig,
}

/// The rendering config of the [`belt`](Belt).
#[derive(Debug, PartialEq)]
pub struct BuckleConfig {
    pub height: f32,
    pub width: f32,
}

impl BeltConfig {
    pub fn get_width(
        &self,
        body_config: &BodyConfig,
        pants_config: &PantsConfig,
        body: &Body,
    ) -> f32 {
        pants_config.get_hip_width(body_config, body) + 2.0 * self.thickness
    }

    pub fn get_top_side_y(&self, config: &BodyConfig) -> f32 {
        config.y_lower + self.y_offset
    }

    pub fn get_top_center_y(&self, body_config: &BodyConfig, pants_config: &PantsConfig) -> f32 {
        self.get_top_side_y(body_config) + pants_config.offset_center
    }

    pub fn get_bottom_side_y(&self, config: &BodyConfig) -> f32 {
        self.get_top_side_y(config) + self.height
    }

    pub fn get_bottom_center_y(&self, body_config: &BodyConfig, pants_config: &PantsConfig) -> f32 {
        self.get_bottom_side_y(body_config) + pants_config.offset_center
    }

    pub fn get_y_values(
        &self,
        body_config: &BodyConfig,
        pants_config: &PantsConfig,
    ) -> (f32, f32, f32, f32) {
        (
            self.get_top_side_y(body_config),
            self.get_top_center_y(body_config, pants_config),
            self.get_bottom_side_y(body_config),
            self.get_bottom_center_y(body_config, pants_config),
        )
    }

    pub fn get_buckle_center(
        &self,
        aabb: &AABB,
        body_config: &BodyConfig,
        pants_config: &PantsConfig,
    ) -> Point2d {
        let center_y = self.get_top_center_y(body_config, pants_config) + self.height * 0.5;
        aabb.get_point(0.5, center_y)
    }

    pub fn get_buckle_aabb(
        &self,
        aabb: &AABB,
        body_config: &BodyConfig,
        pants_config: &PantsConfig,
    ) -> AABB {
        let center = self.get_buckle_center(aabb, body_config, pants_config);
        let size = Size2d::new(
            aabb.convert_to_width(self.buckle.width),
            aabb.convert_to_height(self.buckle.height),
        );
        AABB::with_center(center, size)
    }
}
