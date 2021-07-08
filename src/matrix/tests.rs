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
