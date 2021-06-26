use crate::utils::is_eq_float;

#[allow(unused)]

/// General Tuple to hold a point or a vector
#[derive(Debug, PartialEq)]
struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

#[derive(Debug, PartialEq)]
enum TupleType {
    Point,
    Vector,
}

/// Create a point
fn point(x: f32, y: f32, z: f32) -> Tuple {
    Tuple { x, y, z, w: 1.0 }
}

/// Create a vector
fn vector(x: f32, y: f32, z: f32) -> Tuple {
    Tuple { x, y, z, w: 0.0 }
}

impl Tuple {
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
        Tuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
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
        Tuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl std::ops::Neg for Tuple {
    type Output = Tuple;
    fn neg(self) -> Self::Output {
        assert_ne!(true, self.is_point(), "Cannot negate a point");
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

#[cfg(test)]
mod tests;
