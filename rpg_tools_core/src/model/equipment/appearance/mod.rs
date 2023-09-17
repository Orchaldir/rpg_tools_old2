use crate::model::equipment::appearance::pants::Pants;
use crate::model::equipment::appearance::shirt::Shirt;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

pub mod pants;
pub mod shirt;

#[derive(ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Clothing {
    #[default]
    None,
    Simple {
        pants: Pants,
        shirt: Shirt,
    },
}
