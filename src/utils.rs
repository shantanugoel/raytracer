use num::{Float, NumCast};

/// Compare two floats for equality
pub fn is_eq_float<T>(x: &T, y: &T) -> bool
where
    T: Float,
{
    // Use a custom epsilon instead of Float::epsilon() to limit precision checks
    let epsilon: T = NumCast::from(0.000_01).unwrap();
    if (*x - *y).abs() <= epsilon {
        return true;
    }
    false
}
