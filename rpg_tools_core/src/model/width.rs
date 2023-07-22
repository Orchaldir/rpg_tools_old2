use serde::Serialize;

/// How wide?
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Width {
    Thin,
    Average,
    Wide,
}
