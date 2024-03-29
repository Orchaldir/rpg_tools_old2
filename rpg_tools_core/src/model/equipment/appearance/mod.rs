use crate::model::equipment::appearance::footwear::Footwear;
use crate::model::equipment::appearance::outerwear::Outerwear;
use crate::model::equipment::appearance::pants::Pants;
use crate::model::equipment::appearance::shirt::Shirt;
use macro_ui::ui;
use serde::{Deserialize, Serialize};

pub mod belt;
pub mod eyewear;
pub mod footwear;
pub mod option;
pub mod outerwear;
pub mod pants;
pub mod shirt;

#[derive(ui, Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Clothing {
    #[default]
    None,
    Simple {
        footwear: Footwear,
        pants: Pants,
        shirt: Shirt,
        outerwear: Outerwear,
    },
}
