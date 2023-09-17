use crate::model::character::appearance::body::Body;
use crate::model::character::appearance::head::Head;
use crate::model::length::Length;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

pub mod beard;
pub mod body;
pub mod ear;
pub mod eye;
pub mod hair;
pub mod head;
pub mod mouth;
pub mod skin;

/// How does a [`character`](crate::model::character::Character) look like?
#[derive(ui, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Appearance {
    /// The [`character`](crate::model::character::Character) consists only of a head. E.g. a floating skull
    HeadOnly { head: Head, height: Length },
    /// The [`character`](crate::model::character::Character) is a humanoid. E.g. a human
    Humanoid {
        body: Body,
        head: Head,
        height: Length,
    },
}

impl Appearance {
    pub fn head(head: Head, height: Length) -> Self {
        Self::HeadOnly { head, height }
    }

    pub fn humanoid(body: Body, head: Head, height: Length) -> Self {
        Self::Humanoid { body, head, height }
    }

    pub fn calculate_height(&self) -> Length {
        *match self {
            Appearance::HeadOnly { height, .. } => height,
            Appearance::Humanoid { height, .. } => height,
        }
    }

    /// Mirrors along the center axis.
    pub fn mirror(&self) -> Self {
        match self {
            Appearance::HeadOnly { head, height } => Appearance::HeadOnly {
                head: head.mirror(),
                height: *height,
            },
            Appearance::Humanoid { body, head, height } => Appearance::Humanoid {
                body: *body,
                head: head.mirror(),
                height: *height,
            },
        }
    }
}

impl Default for Appearance {
    fn default() -> Self {
        Appearance::Humanoid {
            body: Body::default(),
            head: Head::default(),
            height: Length::from_metre(1.5),
        }
    }
}
