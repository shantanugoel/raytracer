use std::f32::EPSILON;

pub fn is_eq_float(x: &f32, y: &f32) -> bool {
    if (x - y).abs() < EPSILON {
        return true;
    }
    false
}
