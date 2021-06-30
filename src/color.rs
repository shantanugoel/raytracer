use crate::utils::is_eq_float;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy)]
struct Color {
    red: f32,
    green: f32,
    blue: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color {
            red: r,
            green: g,
            blue: b,
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        is_eq_float(&self.red, &other.red)
            && is_eq_float(&self.green, &other.green)
            && is_eq_float(&self.blue, &other.blue)
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue,
        }
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
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
