use crate::tuple::{Point, Vector};

use super::*;

#[test]
fn test_matrix_creation_default() {
    let num_rows = 2;
    let num_cols = 2;
    let m = Matrix::<f64>::new(num_rows, num_cols);
    let expected_vec = vec![0.0, 0.0, 0.0, 0.0];
    assert_eq!(num_rows * num_cols, m.data.capacity());
    assert_eq!(num_rows * num_cols, m.data.len());
    assert_eq!(expected_vec, m.data);
}

#[test]
fn test_matrix_assignment() {
    let num_rows = 2;
    let num_cols = 3;
    let mut m = Matrix::<f64>::new(num_rows, num_cols);
    let expected_vector = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    for row in 0..num_rows {
        for col in 0..num_cols {
            m[row][col] = expected_vector[row * num_cols + col];
        }
    }
    assert_eq!(expected_vector, m.data);
}

#[test]
fn test_matrix_from_slice() {
    let slice1 = [[1.2, 2.3], [2.1, 3.2]];
    let slice2 = [[1; 3]; 4];
    let m1: Matrix<f64> = Matrix::from(slice1);
    let m2: Matrix<i32> = Matrix::from(slice2);
    for (row, row_val) in slice1.iter().enumerate() {
        for (col, col_val) in row_val.iter().enumerate() {
            assert_eq!(m1[row][col], *col_val);
        }
    }
    for (row, row_val) in slice2.iter().enumerate() {
        for (col, col_val) in row_val.iter().enumerate() {
            assert_eq!(m2[row][col], *col_val);
        }
    }
}

#[test]
fn test_compare() {
    let slice1 = [[1.2, 2.3], [2.1, 3.2]];
    let slice2 = [[1.0; 3]; 4];
    let m1: Matrix<f64> = Matrix::from(slice1);
    let m2: Matrix<f64> = Matrix::from(slice1);
    let m3: Matrix<f64> = Matrix::from(slice2);
    assert_eq!(m1, m2);
    assert_ne!(m1, m3);
}

#[test]
fn test_multiply_matrix() {
    let m1: Matrix<i32> = Matrix::from([[1, 2, 3, 4], [5, 6, 7, 8], [9, 8, 7, 6], [5, 4, 3, 2]]);
    let m2: Matrix<i32> = Matrix::from([[-2, 1, 2, 3], [3, 2, 1, -1], [4, 3, 6, 5], [1, 2, 7, 8]]);
    let expected = Matrix::from([
        [20, 22, 50, 48],
        [44, 54, 114, 108],
        [40, 58, 110, 102],
        [16, 26, 46, 42],
    ]);

    assert_eq!(expected, (m1 * m2).unwrap());

    let m3: Matrix<i32> = Matrix::from([[1, 2, 3, 4], [2, 4, 4, 2], [8, 6, 4, 1], [0, 0, 0, 1]]);
    let slice = [1, 2, 3, 1];
    let m4 = Matrix::from(slice);
    let expected2: Matrix<i32> = Matrix::from([18, 24, 33, 1]);
    assert_eq!(expected2, (m3.clone() * slice).unwrap());
    assert_eq!(expected2, (m3 * m4).unwrap());
}

#[test]
fn test_multiply_matrix_error() {
    let m1: Matrix<i32> = Matrix::from([[1, 2, 3, 4], [5, 6, 7, 8]]);
    let m2: Matrix<i32> = Matrix::from([[-2, 1, 2], [3, 2, 1], [4, 3, 6]]);
    assert!((m1 * m2).is_err());
}

#[test]
fn test_identity() {
    let m1 = Matrix::identity(2, 1.0);
    let expected = Matrix::<f64>::from([[1.0, 0.0], [0.0, 1.0]]);
    assert_eq!(expected, m1);
    let m2 = Matrix::from([[1.0, 2.0], [5.0, 6.0]]);
    assert_eq!(m2, (m2.clone() * m1).unwrap());
}

#[test]
fn test_transpose() {
    let m: Matrix<i32> = Matrix::from([[0, 9, 3, 0], [9, 8, 0, 8], [1, 8, 5, 3], [0, 0, 5, 8]]);
    let expected: Matrix<i32> =
        Matrix::from([[0, 9, 1, 0], [9, 8, 8, 0], [3, 0, 5, 5], [0, 8, 3, 8]]);
    assert_eq!(expected, m.transpose());
    let i = Matrix::identity(4, 1);
    assert_eq!(i, i.transpose());
}

#[test]
fn test_determinant() {
    let m: Matrix<i32> = Matrix::from([[1, 5], [-3, 2]]);
    assert_eq!(17, m.determinant().unwrap());
    let m2: Matrix<i32> = Matrix::from([[1, 5, 6], [-3, 2, 1]]);
    assert!(m2.determinant().is_err());
}

#[test]
fn test_submatrix() {
    let m1 = Matrix::<i32>::from([[1, 5, 0], [-3, 2, 7], [0, 6, -3]]);
    let expected1 = Matrix::<i32>::from([[-3, 2], [0, 6]]);
    assert_eq!(expected1, m1.submatrix(0, 2).unwrap());

    let m2 = Matrix::<i32>::from([[-6, 1, 1, 6], [-8, 5, 8, 6], [-1, 0, 8, 2], [-7, 1, -1, 1]]);
    let expected2 = Matrix::<i32>::from([[-6, 1, 6], [-8, 8, 6], [-7, -1, 1]]);
    assert_eq!(expected2, m2.submatrix(2, 1).unwrap());

    assert!(m1.submatrix(3, 0).is_err());
}

#[test]
fn test_minor() {
    let m = Matrix::<i32>::from([[3, 5, 0], [2, -1, -7], [6, -1, 5]]);
    assert_eq!(25, m.minor(1, 0).unwrap());
}

#[test]
fn test_cofactor() {
    let m = Matrix::<i32>::from([[3, 5, 0], [2, -1, -7], [6, -1, 5]]);
    assert_eq!(-12, m.minor(0, 0).unwrap());
    assert_eq!(-12, m.cofactor(0, 0).unwrap());
    assert_eq!(25, m.minor(1, 0).unwrap());
    assert_eq!(-25, m.cofactor(1, 0).unwrap());

    let m2 = Matrix::<i32>::from([[1, 2, 6], [-5, 8, -4], [2, 6, 4]]);
    assert_eq!(56, m2.cofactor(0, 0).unwrap());
    assert_eq!(12, m2.cofactor(0, 1).unwrap());
    assert_eq!(-46, m2.cofactor(0, 2).unwrap());
    assert_eq!(-196, m2.determinant().unwrap());

    let m3 = Matrix::<i32>::from([[-2, -8, 3, 5], [-3, 1, 7, 3], [1, 2, -9, 6], [-6, 7, 7, -9]]);
    assert_eq!(690, m3.cofactor(0, 0).unwrap());
    assert_eq!(447, m3.cofactor(0, 1).unwrap());
    assert_eq!(210, m3.cofactor(0, 2).unwrap());
    assert_eq!(51, m3.cofactor(0, 3).unwrap());
    assert_eq!(-4071, m3.determinant().unwrap());
}

#[test]
fn test_invertible() {
    let m1 = Matrix::<i32>::from([[6, 4, 4, 4], [5, 5, 7, 6], [4, -9, 3, -7], [9, 1, 7, -6]]);
    let m2 = Matrix::<i32>::from([[-4, 2, -2, -3], [9, 6, 2, 6], [0, -5, 1, -5], [0, 0, 0, 0]]);
    assert_eq!(-2120, m1.determinant().unwrap());
    assert!(m1.is_invertible().unwrap());
    assert_eq!(0, m2.determinant().unwrap());
    assert!(!m2.is_invertible().unwrap());
}

#[test]
fn test_inverse() {
    let m1 = Matrix::<f64>::from([
        [-5., 2., 6., -8.],
        [1., -5., 1., 8.],
        [7., 7., -6., -7.],
        [1., -3., 7., 4.],
    ]);
    let inverted1 = Matrix::<f64>::from([
        [0.21805, 0.45113, 0.24060, -0.04511],
        [-0.80827, -1.45677, -0.44361, 0.52068],
        [-0.07895, -0.22368, -0.05263, 0.19737],
        [-0.52256, -0.81391, -0.30075, 0.30639],
    ]);

    assert_eq!(inverted1, m1.inverse().unwrap().limit_precision(5));

    let m2 = Matrix::<f64>::from([
        [8.0, -5.0, 9.0, 2.0],
        [7.0, 5.0, 6.0, 1.0],
        [-6.0, 0.0, 9.0, 6.0],
        [-3.0, 0.0, -9.0, -4.0],
    ]);
    let inverted2 = Matrix::<f64>::from([
        [-0.15385, -0.15385, -0.28205, -0.53846],
        [-0.07692, 0.12308, 0.02564, 0.03077],
        [0.35897, 0.35897, 0.43590, 0.92308],
        [-0.69231, -0.69231, -0.76923, -1.92308],
    ]);
    assert_eq!(inverted2, m2.inverse().unwrap().limit_precision(5));

    let m3 = Matrix::<f64>::from([
        [9.0, 3.0, 0.0, 9.0],
        [-5.0, -2.0, -6.0, -3.0],
        [-4.0, 9.0, 6.0, 4.0],
        [-7.0, 6.0, 6.0, 2.0],
    ]);
    let inverted3 = Matrix::<f64>::from([
        [-0.04074, -0.07778, 0.14444, -0.22222],
        [-0.07778, 0.03333, 0.36667, -0.33333],
        [-0.02901, -0.14630, -0.10926, 0.12963],
        [0.17778, 0.06667, -0.26667, 0.33333],
    ]);
    assert_eq!(inverted3, m3.inverse().unwrap().limit_precision(5));
}

#[test]
fn test_inverse_multiplication() {
    let m1 = Matrix::<f32>::from([
        [3.0, -9.0, 7.0, 3.0],
        [3.0, -8.0, 2.0, -9.0],
        [-4.0, 4.0, 4.0, 1.0],
        [-6.0, 5.0, -1.0, 1.0],
    ]);

    let m2 = Matrix::<f32>::from([
        [8.0, 2.0, 2.0, 2.0],
        [3.0, -1.0, 7.0, 0.0],
        [7.0, 0.0, 5.0, 4.0],
        [6.0, -2.0, 0.0, 5.0],
    ]);

    let m3 = (m1.clone() * m2.clone()).unwrap();

    assert_eq!(m1, (m3 * m2.inverse().unwrap()).unwrap().limit_precision(5));
}

#[test]
fn test_transformations_translation() {
    let t = Matrix::translation(5.0, -3.0, 2.0);
    let p = Point::new(-3.0, 4.0, 5.0);
    let expected = Point::new(2.0, 1.0, 7.0);
    assert_eq!(expected, (t.clone() * p).unwrap());

    let inv = t.clone().inverse().unwrap();
    let expected2 = Point::new(-8.0, 7.0, 3.0);
    assert_eq!(expected2, (inv * p).unwrap());

    let v = Vector::new(-3.0, 4.0, 5.0);
    assert_eq!(v, (t * v).unwrap());
}

#[test]
fn test_transformations_scaling() {
    let t = Matrix::scaling(2.0, 3.0, 4.0);
    let p = Point::new(-4.0, 6.0, 8.0);
    let expected = Point::new(-8.0, 18.0, 32.0);
    assert_eq!(expected, (t.clone() * p).unwrap());

    let v = Vector::new(-4.0, 6.0, 8.0);
    let expected2 = Vector::new(-8.0, 18.0, 32.0);
    assert_eq!(expected2, (t.clone() * v).unwrap());

    let inv = t.clone().inverse().unwrap();
    let expected3 = Vector::new(-2.0, 2.0, 2.0);
    assert_eq!(expected3, (inv * v).unwrap());
}

#[test]
fn test_transformations_reflection() {
    // Reflection is scaling by a negative value along an axis, here X
    let t = Matrix::scaling(-1.0, 1.0, 1.0);
    let p = Point::new(2.0, 3.0, 4.0);
    let expected = Point::new(-2.0, 3.0, 4.0);
    assert_eq!(expected, (t * p).unwrap());
}

#[test]
fn test_rotation_x() {
    let p = Point::new(0.0, 1.0, 0.0);
    let half_quarter = Matrix::rotation(Axis::X, std::f64::consts::FRAC_PI_4);
    let full_quarter = Matrix::rotation(Axis::X, std::f64::consts::FRAC_PI_2);
    assert_eq!(
        Point::new(0.0, 2.0.sqrt() / 2.0, 2.0.sqrt() / 2.0),
        (half_quarter.clone() * p).unwrap()
    );
    assert_eq!(Point::new(0.0, 0.0, 1.0), (full_quarter * p).unwrap());

    let inv = half_quarter.inverse().unwrap();
    assert_eq!(
        Point::new(0.0, 2.0.sqrt() / 2.0, 2.0.sqrt().neg() / 2.0),
        (inv * p).unwrap()
    );
}

#[test]
fn test_rotation_y() {
    let p = Point::new(0.0, 0.0, 1.0);
    let half_quarter = Matrix::rotation(Axis::Y, std::f64::consts::FRAC_PI_4);
    let full_quarter = Matrix::rotation(Axis::Y, std::f64::consts::FRAC_PI_2);
    assert_eq!(
        Point::new(2.0.sqrt() / 2.0, 0.0, 2.0.sqrt() / 2.0),
        (half_quarter * p).unwrap()
    );
    assert_eq!(Point::new(1.0, 0.0, 0.0), (full_quarter * p).unwrap());
}

#[test]
fn test_rotation_z() {
    let p = Point::new(0.0, 1.0, 0.0);
    let half_quarter = Matrix::rotation(Axis::Z, std::f64::consts::FRAC_PI_4);
    let full_quarter = Matrix::rotation(Axis::Z, std::f64::consts::FRAC_PI_2);
    assert_eq!(
        Point::new(2.0.sqrt().neg() / 2.0, 2.0.sqrt() / 2.0, 0.0),
        (half_quarter * p).unwrap()
    );
    assert_eq!(Point::new(-1.0, 0.0, 0.0), (full_quarter * p).unwrap());
}

#[test]
fn test_shearing() {
    let p = Point::new(2.0, 3.0, 4.0);

    assert_eq!(
        Point::new(5.0, 3.0, 4.0),
        (Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0) * p).unwrap()
    );

    assert_eq!(
        Point::new(6.0, 3.0, 4.0),
        (Matrix::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0) * p).unwrap()
    );

    assert_eq!(
        Point::new(2.0, 5.0, 4.0),
        (Matrix::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0) * p).unwrap()
    );

    assert_eq!(
        Point::new(2.0, 7.0, 4.0),
        (Matrix::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0) * p).unwrap()
    );

    assert_eq!(
        Point::new(2.0, 3.0, 6.0),
        (Matrix::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0) * p).unwrap()
    );

    assert_eq!(
        Point::new(2.0, 3.0, 7.0),
        (Matrix::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0) * p).unwrap()
    );
}
