use crate::utils::is_eq_float;

#[allow(unused)]

/// General Tuple to hold a point or a vector
#[derive(Debug, PartialEq, Clone, Copy)]
struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

#[derive(Debug, PartialEq)]
enum TupleType {
    Point,
    Vector,
}

/// Create a point
fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple { x, y, z, w: 1.0 }
}

/// Create a vector
fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple { x, y, z, w: 0.0 }
}

impl Tuple {
    fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple { x, y, z, w }
    }

    /// Get the type of Tuple
    fn get_tuple_type(self: &Self) -> TupleType {
        match is_eq_float(&self.w, &1.0) {
            true => TupleType::Point,
            false => TupleType::Vector,
        }
    }

    fn is_point(self: &Self) -> bool {
        self.get_tuple_type() == TupleType::Point
    }

    fn is_vector(self: &Self) -> bool {
        self.get_tuple_type() == TupleType::Vector
    }
}

impl std::ops::Add<Tuple> for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Tuple) -> Self::Output {
        assert_ne!(
            true,
            self.is_point() && rhs.is_point(),
            "Cannot add two points"
        );
        Tuple::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl std::ops::Mul<f64> for Tuple {
    type Output = Tuple;
    fn mul(self, rhs: f64) -> Self::Output {
        Tuple::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w)
    }
}

impl std::ops::Mul<Tuple> for f64 {
    type Output = Tuple;
    fn mul(self, rhs: Tuple) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Div<f64> for Tuple {
    type Output = Tuple;
    fn div(self, rhs: f64) -> Self::Output {
        Tuple::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w)
    }
}

impl std::ops::Sub<Tuple> for Tuple {
    type Output = Tuple;
    fn sub(self, rhs: Tuple) -> Self::Output {
        assert_ne!(
            true,
            self.is_vector() && rhs.is_point(),
            "Cannot subtract a point from a vector"
        );
        Tuple::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl std::ops::Neg for Tuple {
    type Output = Tuple;
    fn neg(self) -> Self::Output {
        assert_ne!(true, self.is_point(), "Cannot negate a point");
        Tuple::new(-self.x, -self.y, -self.z, -self.w)
    }
}

#[cfg(test)]
mod tests;
