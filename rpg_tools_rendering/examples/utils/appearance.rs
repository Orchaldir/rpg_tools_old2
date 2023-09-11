use rpg_tools_core::model::character::appearance::beard::Beard;
use rpg_tools_core::model::character::appearance::body::Body;
use rpg_tools_core::model::character::appearance::ear::shape::EarShape;
use rpg_tools_core::model::character::appearance::ear::Ears;
use rpg_tools_core::model::character::appearance::eye::pupil::PupilShape;
use rpg_tools_core::model::character::appearance::eye::shape::EyeShape;
use rpg_tools_core::model::character::appearance::eye::{Eye, Eyes};
use rpg_tools_core::model::character::appearance::hair::Hair;
use rpg_tools_core::model::character::appearance::head::{Head, HeadShape};
use rpg_tools_core::model::character::appearance::mouth::{Mouth, SpecialTeeth, TeethColor};
use rpg_tools_core::model::character::appearance::skin::{Skin, SkinColor};
use rpg_tools_core::model::character::appearance::Appearance;
use rpg_tools_core::model::color::Color;
use rpg_tools_core::model::length::Length;
use rpg_tools_core::model::size::Size::Medium;

pub fn create_head_with_hair(height: Length, hair: &Hair, shape: &HeadShape) -> Appearance {
    Appearance::head(create_head(*shape, hair), height)
}

pub fn create_humanoid_with_hair(height: Length, hair: &Hair, shape: &HeadShape) -> Appearance {
    Appearance::humanoid(
        Body::with_skin(Skin::Skin {
            color: SkinColor::Light,
        }),
        create_head(*shape, hair),
        height,
    )
}

fn create_head(shape: HeadShape, hair: &Hair) -> Head {
    Head {
        ears: Ears::Normal {
            shape: EarShape::Square,
            size: Medium,
        },
        eyes: Eyes::Two {
            eye: Eye::Normal {
                eye_shape: EyeShape::Ellipse,
                pupil_shape: PupilShape::Circle,
                pupil_color: Color::Green,
                background_color: Color::White,
            },
            eyebrows: Default::default(),
            distance: Medium,
        },
        hair: *hair,
        mouth: Mouth::Simple {
            beard: Beard::None,
            width: Medium,
            teeth: SpecialTeeth::None,
            teeth_color: TeethColor::White,
        },
        shape,
        skin: Skin::Skin {
            color: SkinColor::Light,
        },
    }
}
