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
        // Cannot add two points
        assert_ne!(true, self.is_point() && rhs.is_point());
        Tuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

#[test]
fn tuple_test() {
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
fn tuple_point_test() {
    let a = point(4.3, -4.2, 3.1);
    assert!(a.is_point());
    assert_eq!(false, a.is_vector());
}

#[test]
fn tuple_vector_test() {
    let a = vector(4.3, -4.2, 3.1);
    assert!(a.is_vector());
    assert_eq!(false, a.is_point());
}

#[test]
fn tuple_add_test() {
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
