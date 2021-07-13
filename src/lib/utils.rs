use num::Float;

/// Compare two floats for equality
pub fn is_eq_float<T>(x: &T, y: &T) -> bool
where
    T: Float,
{
    if (*x - *y).abs() <= Float::epsilon() {
        return true;
    }
    false
}
