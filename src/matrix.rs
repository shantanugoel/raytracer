use std::usize;

pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T> Matrix<T> {
    pub fn new(rows: usize, cols: usize) -> Matrix<T> {
        Matrix {
            rows,
            cols,
            data: Vec::with_capacity(rows * cols),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::type_name;

    #[test]
    fn test_matrix_creation() {
        let m2x2 = Matrix::<f64>::new(2, 2);
        assert_eq!(4, m2x2.data.capacity());
    }
}
