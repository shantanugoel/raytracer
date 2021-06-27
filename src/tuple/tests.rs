use super::*;

#[test]
fn test_add() {
    // let a1 = point(3.0, -2.0, 5.0);
    let p1 = Point::new(3.0, -2.0, 5.0);
    let v1 = Vector::new(-2.0, 3.0, 1.0);
    let v2 = Vector::new(1.0, 2.0, 3.0);
    let exp_1 = Point::new(1.0, 1.0, 6.0);
    let exp_2 = Vector::new(-1.0, 5.0, 4.0);

    assert_eq!(exp_1, p1 + v1);
    assert_eq!(p1 + v1, v1 + p1);
    assert_eq!(exp_2, v1 + v2);
    assert_eq!(v1 + v2, v2 + v1);
}

#[test]
fn test_sub() {
    let p1_1 = Point::new(3.0, 2.0, 1.0);
    let p2_1 = Point::new(5.0, 6.0, 7.0);
    let expected_1 = Vector::new(-2.0, -4.0, -6.0);
    assert_eq!(expected_1, p1_1 - p2_1);

    let p_2 = Point::new(3.0, 2.0, 1.0);
    let v_2 = Vector::new(5.0, 6.0, 7.0);
    let expected_2 = Point::new(-2.0, -4.0, -6.0);
    assert_eq!(expected_2, p_2 - v_2);

    let v1_3 = Vector::new(3.0, 2.0, 1.0);
    let v2_3 = Vector::new(5.0, 6.0, 7.0);
    let expected_3 = Vector::new(-2.0, -4.0, -6.0);
    assert_eq!(expected_3, v1_3 - v2_3);

    let zero = Vector::new(0.0, 0.0, 0.0);
    let v_4 = Vector::new(1.0, -2.0, 3.0);
    let expected_4 = Vector::new(-1.0, 2.0, -3.0);
    assert_eq!(expected_4, zero - v_4);
}

// #[test]
// fn test_negate() {
//     let a = vector(1.0, -2.0, 3.0);
//     let expected = vector(-1.0, 2.0, -3.0);
//     assert_eq!(expected, -a);
// }

// #[test]
// #[should_panic]
// fn test_fail_point_negate() {
//     let a = point(1.0, -2.0, 3.0);
//     let _ = -a;
// }

// #[test]
// fn test_multiply() {
//     let a = Tuple::new(1.0, -2.0, 3.0, 1.0);
//     let expected_1 = Tuple::new(3.5, -7.0, 10.5, 1.0);
//     let expected_2 = Tuple::new(0.5, -1.0, 1.5, 1.0);
//     assert_eq!(expected_1, a * 3.5);
//     assert_eq!(expected_2, a * 0.5);
// }

// #[test]
// fn test_divide() {
//     let a = Tuple::new(1.0, -2.0, 3.0, 1.0);
//     let expected_2 = Tuple::new(0.5, -1.0, 1.5, 1.0);
//     assert_eq!(expected_2, a / 2.0);
// }

#[test]
fn test_magnitude() {
    let test_vector = vec![
        Vector::new(1.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 0.0, 1.0),
    ];
    for v in test_vector {
        assert!(is_eq_float(&1.0, &v.magnitude()))
    }

    let v1 = Vector::new(1.0, 2.0, 3.0);
    let v2 = Vector::new(-1.0, -2.0, -3.0);
    assert!(is_eq_float(&14.0_f64.sqrt(), &v1.magnitude()));
    assert!(is_eq_float(&v1.magnitude(), &v2.magnitude()));
}

#[test]
fn test_normalize() {
    let test = vec![Vector::new(4.0, 0.0, 0.0), Vector::new(1.0, 2.0, 3.0)];
    let expected = vec![
        Vector::new(1.0, 0.0, 0.0),
        Vector::new(
            1.0 / 14.0_f64.sqrt(),
            2.0 / 14.0_f64.sqrt(),
            3.0 / 14.0_f64.sqrt(),
        ),
    ];
    for i in 0..test.len() {
        assert_eq!(expected[i], test[i].normalize());
    }

    assert_eq!(1.0, test[1].normalize().magnitude());
}

#[test]
fn test_dot() {
    let a = Vector::new(1.0, 2.0, 3.0);
    let b = Vector::new(2.0, 3.0, 4.0);
    assert_eq!(20.0, a.dot(b));
}

#[test]
fn test_cross() {
    let a = Vector::new(1.0, 2.0, 3.0);
    let b = Vector::new(2.0, 3.0, 4.0);
    let expected_1 = Vector::new(-1.0, 2.0, -1.0);
    let expected_2 = Vector::new(1.0, -2.0, 1.0);
    assert_eq!(expected_1, a.cross(b));
    assert_eq!(expected_2, b.cross(a));
}
