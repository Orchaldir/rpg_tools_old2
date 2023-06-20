/// An orientation in 2d.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Orientation(f32);

impl Orientation {
    /// Creates an orientation from degree.
    ///
    /// ```
    ///# use rpg_tools_rendering::math::orientation::Orientation;
    /// assert_eq!(Orientation::from_degree(90.0).to_degree(), 90.0);
    /// ```
    pub fn from_degree(degree: f32) -> Self {
        Self(degree)
    }

    /// Creates an orientation from degree.
    ///
    /// ```
    ///# use std::f32::consts::PI;
    ///# use rpg_tools_rendering::math::orientation::Orientation;
    /// assert_eq!(Orientation::from_radians(PI).to_radians(), PI);
    /// ```
    pub fn from_radians(radians: f32) -> Self {
        Self(radians.to_degrees())
    }

    /// Returns the orientation as degree.
    pub fn to_degree(&self) -> f32 {
        self.0
    }

    /// Returns the orientation as radians.
    pub fn to_radians(&self) -> f32 {
        self.0.to_radians()
    }
}
