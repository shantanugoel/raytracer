#[allow(unused)]
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

fn get_tuple_type(input: &Tuple) -> TupleType {
    if f32::EPSILON < 1.0 - input.w {
        TupleType::Vector
    } else {
        TupleType::Point
    }
}

fn point(x: f32, y: f32, z: f32) -> Tuple {
    Tuple { x, y, z, w: 1.0 }
}

fn vector(x: f32, y: f32, z: f32) -> Tuple {
    Tuple { x, y, z, w: 0.0 }
}

#[test]
fn tuple_test() {
    let a = Tuple {
        x: 4.3,
        y: -4.2,
        z: 3.1,
        w: 1.0,
    };
    assert_eq!(TupleType::Point, get_tuple_type(&a));
    assert_ne!(TupleType::Vector, get_tuple_type(&a));

    let b = Tuple {
        x: 4.3,
        y: -4.2,
        z: 3.1,
        w: 0.0,
    };
    assert_eq!(TupleType::Vector, get_tuple_type(&b));
    assert_ne!(TupleType::Point, get_tuple_type(&b));
}

#[test]
fn tuple_point_test() {
    let a = point(4.3, -4.2, 3.1);
    assert_eq!(TupleType::Point, get_tuple_type(&a));
    assert_ne!(TupleType::Vector, get_tuple_type(&a));
}

#[test]
fn tuple_vector_test() {
    let a = vector(4.3, -4.2, 3.1);
    assert_eq!(TupleType::Vector, get_tuple_type(&a));
    assert_ne!(TupleType::Point, get_tuple_type(&a));
}
