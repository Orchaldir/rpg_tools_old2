use crate::model::character::appearance::body::Body;
use crate::model::character::appearance::head::Head;
use crate::model::length::Length;

pub mod body;
pub mod eye;
pub mod head;
pub mod skin;

/// The relative size of an appearance feature.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Size {
    Low,
    Medium,
    High,
}

/// How does a [`character`](crate::model::character::Character) look like?
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
}
