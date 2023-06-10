/// Interpolates between 2 i32 linearly.
///
/// ```
///# use rpg_tools_rendering::math::interpolate::lerp;
/// assert_eq!(lerp(100, 200, 0.0), 100);
/// assert_eq!(lerp(100, 200, 0.5), 150);
/// assert_eq!(lerp(100, 200, 1.0), 200);
/// ```
pub fn lerp(start: i32, end: i32, factor: f32) -> i32 {
    if factor >= 1.0 {
        return end;
    } else if factor <= 0.0 {
        return start;
    }

    if end >= start {
        let diff = (end - start) as f32;
        return start + (diff * factor) as i32;
    }

    let diff = (start - end) as f32;

    start - (diff * factor) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lerp_from_high_to_low() {
        assert_eq!(lerp(200, 100, 0.0), 200);
        assert_eq!(lerp(200, 100, 0.5), 150);
        assert_eq!(lerp(200, 100, 1.0), 100);
    }

    #[test]
    fn test_lerp_negative() {
        assert_eq!(lerp(-200, -100, 0.0), -200);
        assert_eq!(lerp(-200, -100, 0.5), -150);
        assert_eq!(lerp(-200, -100, 1.0), -100);
    }

    #[test]
    fn test_lerp_with_negative_factor() {
        assert_eq!(lerp(100, 200, -0.5), 100);
        assert_eq!(lerp(200, 100, -0.5), 200);
    }

    #[test]
    fn test_lerp_with_too_high_factor() {
        assert_eq!(lerp(100, 200, 2.0), 200);
        assert_eq!(lerp(200, 100, 2.5), 100);
    }
}
