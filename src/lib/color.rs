use num::ToPrimitive;

use crate::utils::is_eq_float;
use std::ops::{Add, Mul, Sub};

// Common Colors
pub enum CommonColor {
    Red,
    Green,
    Blue,
    Black,
    White,
}

impl CommonColor {
    pub fn value(&self) -> Color {
        match *self {
            CommonColor::Red => Color::new(1.0, 0.0, 0.0),
            CommonColor::Green => Color::new(0.0, 1.0, 0.0),
            CommonColor::Blue => Color::new(0.0, 0.0, 1.0),
            CommonColor::Black => Color::new(0.0, 0.0, 0.0),
            CommonColor::White => Color::new(1.0, 1.0, 1.0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }

    pub fn to_scaled_rgb_string(self) -> String {
        format!(
            "{} {} {}",
            scale_to_rgb(self.r),
            scale_to_rgb(self.g),
            scale_to_rgb(self.b)
        )
    }

    pub fn limit_precision(&self, num_places: i32) -> Self {
        let mut c = Color::new(0.0, 0.0, 0.0);
        let factor = 10.0_f64.powi(num_places);
        c.r = (self.r * factor).round() / factor;
        c.g = (self.g * factor).round() / factor;
        c.b = (self.b * factor).round() / factor;
        c
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

impl Mul<f64> for Color {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

// Scale incoming color float to a 0-255 u8 range
fn scale_to_rgb(c: f64) -> u8 {
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
