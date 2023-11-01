use rpg_tools_core::model::equipment::appearance::eyewear::FrameType;

/// The rendering config of the [`eyewear`](rpg_tools_core::model::equipment::appearance::eyewear::Eyewear).
#[derive(Debug, PartialEq)]
pub struct EyewearConfig {
    pub radius_factor: f32,
    pub thickness_horn: f32,
    pub thickness_rimmed: f32,
    pub thickness_wire: f32,
}

impl EyewearConfig {
    pub fn get_radius(&self, eye_radius: u32) -> u32 {
        (eye_radius as f32 * self.radius_factor) as u32
    }

    pub fn get_bridge_thickness(&self, frame_type: FrameType) -> f32 {
        match frame_type {
            FrameType::Rimless => self.thickness_wire,
            _ => self.get_frame_thickness(frame_type),
        }
    }

    pub fn get_frame_thickness(&self, frame_type: FrameType) -> f32 {
        match frame_type {
            FrameType::Horn => self.thickness_horn,
            FrameType::FullRimmed => self.thickness_rimmed,
            FrameType::Wire => self.thickness_wire,
            FrameType::Rimless => 0.0,
        }
    }
}
