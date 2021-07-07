use std::{
    ops::{Index, IndexMut},
    usize,
};

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
}