use num::ToPrimitive;

use crate::utils::is_eq_float;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b }
    }

    pub fn to_string(self: &Self) -> String {
        format!(
            "{} {} {}",
            scale_to_rgb(self.r),
            scale_to_rgb(self.g),
            scale_to_rgb(self.b)
        )
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::new(0.0, 0.0, 0.0)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        is_eq_float(&self.r, &other.r)
            && is_eq_float(&self.g, &other.g)
            && is_eq_float(&self.b, &other.b)
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

// Scale incoming color float to a 0-255 u8 range
fn scale_to_rgb(c: f32) -> u8 {
    let mut v = c * 256.0;
    v = v.clamp(0.0, 255.0);
    v.to_u8().expect("Failure converting color value to u8")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_math_ops() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let e1 = Color::new(1.6, 0.7, 1.0);
        assert_eq!(e1, c1 + c2);
    }
}
