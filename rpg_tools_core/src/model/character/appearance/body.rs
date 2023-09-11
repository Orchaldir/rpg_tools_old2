use crate::model::character::appearance::skin::Skin;
use crate::model::width::Width;
use macro_convert::Convert;
use macro_core::parser::{get_enum, UiParser};
use macro_core::visitor::{UiVisitor, UI};
use macro_ui::ui;
use serde::{Deserialize, Serialize};

/// How does the humanoid body look like?
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Body {
    pub shape: BodyShape,
    /// How wide is the body?
    pub width: Width,
    pub skin: Skin,
}

impl Default for Body {
    fn default() -> Self {
        Body::with_skin(Skin::default())
    }
}

impl Body {
    pub fn with_skin(skin: Skin) -> Body {
        Self {
            shape: BodyShape::Rectangle,
            width: Width::Average,
            skin,
        }
    }
}

/// The body shape is defined by the ratio between shoulders, waist & hips.
#[derive(Convert, ui, Default, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BodyShape {
    /// The body gets wider from the shoulders to the hips.
    Fat,
    /// The shoulders & hips are wider than the waist.
    Hourglass,
    /// The body gets wider from the hips to the shoulders.
    Muscular,
    /// The shoulders, waist & hips are equally wide.
    #[default]
    Rectangle,
}
