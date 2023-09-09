use crate::rendering::config::hair::hairline::HairlineConfig;
use crate::rendering::config::hair::ponytail::PonytailConfig;
use crate::rendering::config::hair::short::ShortHairConfig;

pub mod hairline;
pub mod ponytail;
pub mod short;

#[derive(Debug, PartialEq)]
pub struct HairConfig {
    pub hairline: HairlineConfig,
    pub ponytail: PonytailConfig,
    pub short: ShortHairConfig,
}
