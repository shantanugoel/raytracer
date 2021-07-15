use std::{
    ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Neg, Sub},
    usize,
};

use num::{Float, Integer, NumCast, One, Zero};

use thiserror::Error;

use crate::tuple::IsTuple;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl<T> Matrix<T>
where
    T: Default + Copy,
{
    pub fn new(rows: usize, cols: usize) -> Matrix<T> {
        let mut data: Vec<T> = Vec::<T>::with_capacity(rows * cols);
        data.resize_with(rows * cols, T::default);
        Matrix { rows, cols, data }
    }

    /// Returns a matrix with diagonal elementts initialized by `value`
    /// Supply 1.0 as the value for a true identity matrix
    pub fn identity(dimensions: usize, value: T) -> Self {
        let mut m: Matrix<T> = Matrix::new(dimensions, dimensions);
        for index in 0..dimensions {
            m[index][index] = value;
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
        T: Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Neg<Output = T>,
    {
        // Determinant is only for square matrices
        if self.rows != self.cols {
            let error = format!("Expected square matrix, got {}x{}", self.rows, self.cols);
            return Err(MatrixError::InvalidArgument(error));
        }

        let mut det = T::default();
        if self.cols == 2 {
            det = self[0][0] * self[1][1] - self[0][1] * self[1][0];
        } else {
            for col in 0..self.cols {
                det = det + self[0][col] * self.cofactor(0, col)?;
            }
        }
        Ok(det)
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
        T: Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Neg<Output = T>,
    {
        self.submatrix(row, col)?.determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> Result<T, MatrixError>
    where
        T: Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Neg<Output = T>,
    {
        let mut c = self.minor(row, col)?;
        if (row + col).is_odd() {
            c = c.neg();
        }
        Ok(c)
    }

    pub fn is_invertible(&self) -> Result<bool, MatrixError>
    where
        T: Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Neg<Output = T> + Zero,
    {
        Ok(!self.determinant()?.is_zero())
    }

    pub fn inverse(&self) -> Result<Self, MatrixError>
    where
        T: Mul<Output = T>
            + Sub<Output = T>
            + Add<Output = T>
            + Neg<Output = T>
            + Div<Output = T>
            + Zero,
    {
        if !(self.is_invertible()?) {
            return Err(MatrixError::InvalidArgument(String::from(
                "Provided matrix is not invertible",
            )));
        }
        let mut inverted = Matrix::new(self.rows, self.cols);
        let determinant = self.determinant()?;
        for row in 0..self.rows {
            for col in 0..self.cols {
                let c = self.cofactor(row, col)?;
                inverted[col][row] = c / determinant;
            }
        }

        Ok(inverted)
    }

    pub fn limit_precision(&self, num_places: i32) -> Self
    where
        T: Float,
    {
        let mut m = Matrix::<T>::new(self.rows, self.cols);
        let factor: T = NumCast::from(10.0.powi(num_places)).unwrap();
        for row in 0..self.rows {
            for col in 0..self.cols {
                m[row][col] = (self[row][col] * factor).round() / factor;
            }
        }
        m
    }

    pub fn translation(x: T, y: T, z: T) -> Matrix<T>
    where
        T: One,
    {
        // Supporting only 4x4 translation matrix
        let dimensions = 4;
        let mut m = Matrix::identity(dimensions, T::one());
        m[0][3] = x;
        m[1][3] = y;
        m[2][3] = z;
        m
    }

    pub fn translate(self, x: T, y: T, z: T) -> Result<Matrix<T>, MatrixError>
    where
        T: One + Default + Copy + Mul<Output = T> + AddAssign,
    {
        &Matrix::translation(x, y, z) * &self
    }

    pub fn scaling(x: T, y: T, z: T) -> Matrix<T>
    where
        T: One,
    {
        // Supporting only 4x4 scaling matrix
        let dimensions = 4;
        let mut m = Matrix::identity(dimensions, T::one());
        m[0][0] = x;
        m[1][1] = y;
        m[2][2] = z;
        m
    }

    pub fn scale(self, x: T, y: T, z: T) -> Result<Matrix<T>, MatrixError>
    where
        T: One + Default + Copy + Mul<Output = T> + AddAssign,
    {
        &Matrix::scaling(x, y, z) * &self
    }

    pub fn rotation(axis: Axis, radian: T) -> Matrix<T>
    where
        T: Float + One,
    {
        let cosr = radian.cos();
        let sinr = radian.sin();
        let mut m = Matrix::<T>::identity(4, T::one());
        match axis {
            Axis::X => {
                m[1][1] = cosr;
                m[1][2] = sinr.neg();
                m[2][1] = sinr;
                m[2][2] = cosr;
            }
            Axis::Y => {
                m[0][0] = cosr;
                m[2][0] = sinr.neg();
                m[0][2] = sinr;
                m[2][2] = cosr;
            }
            Axis::Z => {
                m[0][0] = cosr;
                m[0][1] = sinr.neg();
                m[1][0] = sinr;
                m[1][1] = cosr;
            }
        }
        m
    }

    pub fn rotate(self, axis: Axis, radian: T) -> Result<Matrix<T>, MatrixError>
    where
        T: Float + One + Default + Copy + Mul<Output = T> + AddAssign,
    {
        &Matrix::rotation(axis, radian) * &self
    }

    pub fn shearing(xy: T, xz: T, yx: T, yz: T, zx: T, zy: T) -> Matrix<T>
    where
        T: Float + One,
    {
        let mut m = Matrix::<T>::identity(4, T::one());
        m[0][1] = xy;
        m[0][2] = xz;
        m[1][0] = yx;
        m[1][2] = yz;
        m[2][0] = zx;
        m[2][1] = zy;
        m
    }

    pub fn shear(self, xy: T, xz: T, yx: T, yz: T, zx: T, zy: T) -> Result<Matrix<T>, MatrixError>
    where
        T: Float + One + Default + Copy + Mul<Output = T> + AddAssign,
    {
        &Matrix::shearing(xy, xz, yx, yz, zx, zy) * &self
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

impl<U> From<U> for Matrix<f64>
where
    U: IsTuple,
{
    fn from(t: U) -> Self {
        Matrix::from([t.tuple().x, t.tuple().y, t.tuple().z, t.tuple().w])
    }
}

impl<T> Mul for &Matrix<T>
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

// Multiplying matrix with a point or vector
impl<U> Mul<U> for &Matrix<f64>
where
    U: IsTuple,
{
    type Output = Result<U, MatrixError>;
    fn mul(self, rhs: U) -> Self::Output {
        //let mut output = U::new(0.0, 0.0, 0.0);
        let m = Matrix::from(rhs);
        let temp = (self * &m)?;
        Ok(U::new(temp[0][0], temp[1][0], temp[2][0]))
    }
}

impl<T, const R: usize> Mul<[T; R]> for &Matrix<T>
where
    T: Default + Copy + Mul<Output = T> + AddAssign,
{
    type Output = Result<Matrix<T>, MatrixError>;
    fn mul(self, rhs: [T; R]) -> Self::Output {
        let m2 = Matrix::from(rhs);
        self * &m2
    }
}

#[cfg(test)]
mod tests;
