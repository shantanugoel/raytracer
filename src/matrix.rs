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
        let mut m2x2 = Matrix::<f64>::new(2, 2);
        m2x2[0][0] = 1.0;
        m2x2[0][1] = 2.0;
        m2x2[1][0] = 3.0;
        m2x2[1][1] = 4.0;
        assert_eq!(4, m2x2.data.capacity());
    }

    #[test]
    fn test_matrix_assignment() {
        let num_rows = 2;
        let num_cols = 3;
        let mut m2x2 = Matrix::<f64>::new(num_rows, num_cols);
        let expected_vector = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        for row in 0..num_rows {
            for col in 0..num_cols {
                m2x2[row][col] = expected_vector[row * num_cols + col];
            }
        }
        assert_eq!(expected_vector, m2x2.data);
    }
}
