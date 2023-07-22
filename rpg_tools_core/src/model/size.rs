use serde::Serialize;

/// The relative size of something.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Size {
    Low,
    Medium,
    High,
}
