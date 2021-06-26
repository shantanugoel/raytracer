use std::f64::EPSILON;

/// Compare two floats for equality
pub fn is_eq_float(x: &f64, y: &f64) -> bool {
    if (x - y).abs() < EPSILON {
        return true;
    }
    false
}
