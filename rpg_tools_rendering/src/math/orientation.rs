use std::ops::Add;

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

    /// Splits a full rotation into several equal parts.
    ///
    /// ```
    ///# use rpg_tools_rendering::math::orientation::Orientation;
    /// assert_eq!(Orientation::split(10).to_degree(), 36.0);
    /// ```
    pub fn split(parts: u32) -> Self {
        Self::from_degree(360.0 / parts as f32)
    }

    /// Returns the orientation as degree.
    pub fn to_degree(&self) -> f32 {
        self.0
    }

    /// Returns the orientation as radians.
    pub fn to_radians(&self) -> f32 {
        self.0.to_radians()
    }

    /// Returns the cosinus of the orientation.
    pub fn cos(&self) -> f32 {
        self.0.to_radians().cos()
    }

    /// Returns the sinus of the orientation.
    pub fn sin(&self) -> f32 {
        self.0.to_radians().sin()
    }

    /// Returns the orientation normal to the current one.
    ///
    /// ```
    ///# use rpg_tools_rendering::math::orientation::Orientation;
    /// assert_eq!(Orientation::from_degree(180.0).normal(), Orientation::from_degree(270.0));
    /// ```
    pub fn normal(&self) -> Orientation {
        Self::from_degree(self.0 + 90.0)
    }

    /// Rotates half around the circle.
    ///
    /// ```
    ///# use rpg_tools_rendering::math::orientation::Orientation;
    /// assert_eq!(Orientation::from_degree(0.0).inverse(), Orientation::from_degree(180.0));
    /// ```
    pub fn inverse(&self) -> Orientation {
        Self::from_degree(self.0 + 180.0)
    }
}

impl Add<Orientation> for Orientation {
    type Output = Orientation;

    /// Adds 2 points together.
    ///
    /// ```
    ///# use rpg_tools_rendering::math::orientation::Orientation;
    /// let a = Orientation::from_degree(10.0);
    /// let b = Orientation::from_degree(30.0);
    /// let result = Orientation::from_degree(40.0);
    ///
    /// assert_eq!(a + b, result);
    /// assert_eq!(b + a, result);
    /// ```
    fn add(self, other: Orientation) -> Orientation {
        Orientation::from_degree(self.0 + other.0)
    }
}
