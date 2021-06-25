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
            x: -1.0 * self.x,
            y: -1.0 * self.y,
            z: -1.0 * self.z,
            w: self.w, // No need to negate this since it will remain 0 for vector
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let a = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0,
        };
        assert!(a.is_point());
        assert_eq!(false, a.is_vector());

        let b = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 0.0,
        };
        assert!(b.is_vector());
        assert_eq!(false, b.is_point());
    }

    #[test]
    fn test_point() {
        let a = point(4.3, -4.2, 3.1);
        assert!(a.is_point());
        assert_eq!(false, a.is_vector());
    }

    #[test]
    fn test_vector() {
        let a = vector(4.3, -4.2, 3.1);
        assert!(a.is_vector());
        assert_eq!(false, a.is_point());
    }

    #[test]
    fn test_add() {
        let a1 = point(3.0, -2.0, 5.0);
        let a2 = vector(-2.0, 3.0, 1.0);
        let expected_tuple = Tuple {
            x: 1.0,
            y: 1.0,
            z: 6.0,
            w: 1.0,
        };
        assert_eq!(expected_tuple, a1 + a2);
    }

    #[test]
    #[should_panic]
    fn test_fail_to_add_points() {
        let a1 = point(3.0, -2.0, 5.0);
        let a2 = point(-2.0, 3.0, 1.0);
        let _ = a1 + a2;
    }

    #[test]
    fn test_sub() {
        let p1_1 = point(3.0, 2.0, 1.0);
        let p2_1 = point(5.0, 6.0, 7.0);
        let expected_1 = vector(-2.0, -4.0, -6.0);
        assert_eq!(expected_1, p1_1 - p2_1);

        let p_2 = point(3.0, 2.0, 1.0);
        let v_2 = vector(5.0, 6.0, 7.0);
        let expected_2 = point(-2.0, -4.0, -6.0);
        assert_eq!(expected_2, p_2 - v_2);

        let v1_3 = vector(3.0, 2.0, 1.0);
        let v2_3 = vector(5.0, 6.0, 7.0);
        let expected_3 = vector(-2.0, -4.0, -6.0);
        assert_eq!(expected_3, v1_3 - v2_3);

        let zero = vector(0.0, 0.0, 0.0);
        let v_4 = vector(1.0, -2.0, 3.0);
        let expected_4 = vector(-1.0, 2.0, -3.0);
        assert_eq!(expected_4, zero - v_4);
    }

    #[test]
    #[should_panic]
    fn test_fail_sub_point_from_vector() {
        let a1 = vector(3.0, -2.0, 5.0);
        let a2 = point(-2.0, 3.0, 1.0);
        let _ = a1 - a2;
    }

    #[test]
    fn test_negate() {
        let a = vector(1.0, -2.0, 3.0);
        let expected = vector(-1.0, 2.0, -3.0);
        assert_eq!(expected, -a);
    }

    #[test]
    #[should_panic]
    fn test_fail_point_negate() {
        let a = point(1.0, -2.0, 3.0);
        let _ = -a;
    }
}
