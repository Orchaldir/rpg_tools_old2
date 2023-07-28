use crate::rendering::config::hair::hairline::HairlineConfig;
use crate::rendering::config::hair::short::ShortHairConfig;

pub mod hairline;
pub mod short;

#[derive(Debug, PartialEq)]
pub struct HairConfig {
    pub hairline: HairlineConfig,
    pub short: ShortHairConfig,
}
