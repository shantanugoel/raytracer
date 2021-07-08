use std::{
    ops::{AddAssign, Index, IndexMut, Mul, Sub},
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
    T: Default + Copy,
{
    pub fn new(rows: usize, cols: usize) -> Matrix<T> {
        let mut data: Vec<T> = Vec::<T>::with_capacity(rows * cols);
        data.resize_with(rows * cols, Default::default);
        Matrix { rows, cols, data }
    }

    /// Returns a matrix with diagonal elementts initialized by `value`
    /// Supply 1.0 as the value for a true identity matrix
    pub fn identity(dimensions: usize, value: T) -> Self
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
        self.data[row_index * self.cols..(row_index + 1) * self.cols].iter()
    }

    pub fn iter(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        (0..self.rows).map(move |row_index| self.row_iter(row_index))
    }

    pub fn transpose(&self) -> Self {
        let mut m = Matrix::new(self.rows, self.cols);
        for row in 0..self.rows {
            for col in 0..self.cols {
                m[col][row] = self[row][col];
                m[row][col] = self[col][row];
            }
        }
        m
    }

    pub fn determinant(&self) -> Result<T, MatrixError>
    where
        T: Mul<Output = T> + Sub<Output = T>,
    {
        // Determinant is only for 2x2 matrices
        if self.rows != 2 || self.cols != 2 {
            let error = format!("Expected 2x2 matrix, got {}x{}", self.rows, self.cols);
            return Err(MatrixError::InvalidArgument(error));
        }

        Ok(self[0][0] * self[1][1] - self[0][1] * self[1][0])
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Result<Self, MatrixError> {
        if row >= self.rows || col >= self.cols {
            let error = format!(
                "Row ({}) or column ({}) larger than matrix size {}x{}",
                row, col, self.rows, self.cols
            );
            return Err(MatrixError::InvalidArgument(error));
        }

        let mut m = Matrix::new(self.rows - 1, self.cols - 1);

        let mut submatrix_row = 0;
        for r in 0..self.rows {
            if r == row {
                continue;
            }
            let mut submatrix_col = 0;
            for c in 0..self.cols {
                if c == col {
                    continue;
                }
                m[submatrix_row][submatrix_col] = self[r][c];
                submatrix_col += 1;
            }
            submatrix_row += 1;
        }
        Ok(m)
    }

    pub fn minor(&self, row: usize, col: usize) -> Result<T, MatrixError>
    where
        T: Mul<Output = T> + Sub<Output = T>,
    {
        //TODO Implement for larger than 3x3
        self.submatrix(row, col)?.determinant()
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
mod tests;
