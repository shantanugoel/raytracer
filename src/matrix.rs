use std::{
    ops::{AddAssign, Index, IndexMut, Mul},
    usize,
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MatrixError {
    #[error("Invalid arguments {0}")]
    InvalidArgument(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T> Matrix<T>
where
    T: Default,
{
    pub fn new(rows: usize, cols: usize) -> Matrix<T> {
        let mut data: Vec<T> = Vec::<T>::with_capacity(rows * cols);
        data.resize_with(rows * cols, Default::default);
        Matrix { rows, cols, data }
    }

    /// Returns a matrix with diagonal elementts initialized by `value`
    /// Supply 1.0 as the value for a true identity matrix
    pub fn identity(dimensions: usize, value: T) -> Matrix<T>
    where
        T: Clone,
    {
        let mut m: Matrix<T> = Matrix::new(dimensions, dimensions);
        for index in 0..dimensions {
            m[index][index] = value.clone();
        }
        m
    }

    pub fn num_rows(&self) -> usize {
        self.rows
    }

    pub fn num_cols(&self) -> usize {
        self.cols
    }

    fn row_iter(&self, row_index: usize) -> impl Iterator<Item = &T> {
        println!("{} {}", row_index, (row_index + 1) * self.cols);
        self.data[row_index * self.cols..(row_index + 1) * self.cols].iter()
    }

    pub fn iter(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        (0..self.rows).map(move |row_index| self.row_iter(row_index))
    }
}

impl<T> Index<usize> for Matrix<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index * self.cols..(index + 1) * self.cols]
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index * self.cols..(index + 1) * self.cols]
    }
}

impl<T, const R: usize, const C: usize> From<[[T; C]; R]> for Matrix<T>
where
    T: Default + Copy,
{
    fn from(x: [[T; C]; R]) -> Self {
        let mut m = Matrix::<T>::new(R, C);
        for r in 0..R {
            for c in 0..C {
                m[r][c] = x[r][c];
            }
        }
        m
    }
}

// Treats single dimension slices as multi-row/single-column matrix.
impl<T, const R: usize> From<[T; R]> for Matrix<T>
where
    T: Default + Copy,
{
    fn from(x: [T; R]) -> Self {
        let mut m = Matrix::<T>::new(R, 1);
        for r in 0..R {
            m[r][0] = x[r];
        }
        m
    }
}

impl<T> Mul for Matrix<T>
where
    T: Default + Copy + Mul<Output = T> + AddAssign,
{
    type Output = Result<Matrix<T>, MatrixError>;

    fn mul(self, rhs: Self) -> Self::Output {
        let num_rows1 = self.rows;
        let num_cols1 = self.cols;
        let num_rows2 = rhs.rows;
        let num_cols2 = rhs.cols;

        if num_cols1 != num_rows2 {
            return Err(MatrixError::InvalidArgument(
                format!("Incompatible matrices. Matrix 1 Col size {} and Matrix 2 Row size {} are not equal",
                 num_cols1, num_rows2)));
        }

        let mut output = Matrix::new(num_rows1, num_cols2);
        for row in 0..num_rows1 {
            for col in 0..num_cols2 {
                for index in 0..num_cols1 {
                    output[row][col] += self[row][index] * rhs[index][col];
                }
            }
        }
        Ok(output)
    }
}

impl<T, const R: usize> Mul<[T; R]> for Matrix<T>
where
    T: Default + Copy + Mul<Output = T> + AddAssign,
{
    type Output = Result<Matrix<T>, MatrixError>;
    fn mul(self, rhs: [T; R]) -> Self::Output {
        let m2 = Matrix::from(rhs);
        self * m2
    }
}

#[cfg(test)]
mod tests {
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
        let m1: Matrix<i32> =
            Matrix::from([[1, 2, 3, 4], [5, 6, 7, 8], [9, 8, 7, 6], [5, 4, 3, 2]]);
        let m2: Matrix<i32> =
            Matrix::from([[-2, 1, 2, 3], [3, 2, 1, -1], [4, 3, 6, 5], [1, 2, 7, 8]]);
        let expected = Matrix::from([
            [20, 22, 50, 48],
            [44, 54, 114, 108],
            [40, 58, 110, 102],
            [16, 26, 46, 42],
        ]);

        assert_eq!(expected, (m1 * m2).unwrap());

        let m3: Matrix<i32> =
            Matrix::from([[1, 2, 3, 4], [2, 4, 4, 2], [8, 6, 4, 1], [0, 0, 0, 1]]);
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
        let m = Matrix::identity(2, 1.0);
        let expected = Matrix::<f64>::from([[1.0, 0.0], [0.0, 1.0]]);
        assert_eq!(expected, m);
    }
}
