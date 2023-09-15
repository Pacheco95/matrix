use crate::Matrix;

use std::ops::{Add, Mul};

impl<T, const ROWS: usize, const COLS: usize, const OTHER_COLS: usize>
    Mul<Matrix<T, COLS, OTHER_COLS>> for Matrix<T, ROWS, COLS>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + Default,
{
    type Output = Matrix<T, ROWS, OTHER_COLS>;

    #[inline]
    fn mul(self, rhs: Matrix<T, COLS, OTHER_COLS>) -> Self::Output {
        let mut result: Self::Output = unsafe {
            if self.dim() == rhs.dim() {
                // If both matrices are square and with the same dimensions there
                // is no need to allocate memory for another matrix
                *(&rhs as *const Matrix<T, COLS, OTHER_COLS> as *const Self::Output)
            } else {
                Matrix::default()
            }
        };

        for i in 0..ROWS {
            for j in 0..OTHER_COLS {
                let mut sum = T::default();

                for k in 0..COLS {
                    sum = sum + self[i][k] * rhs[k][j];
                }

                result[i][j] = sum;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_multiplicate_matrices_with_different_dimensions() {
        #[rustfmt::skip]
        let a = Matrix([
            [1, 2],
            [3, 4]
        ]);

        #[rustfmt::skip]
        let b = Matrix([
            [1, 2, 3],
            [4, 5, 6]
        ]);

        let actual = a * b;

        let expected = Matrix([[9, 12, 15], [19, 26, 33]]);

        assert_eq!(actual, expected)
    }

    #[test]
    fn it_should_multiplicate_matrices_with_same_dimensions() {
        let identity: Matrix<i32, 4, 4> = Matrix::identity();
        let ones: Matrix<i32, 4, 4> = Matrix::ones();
        assert_eq!(identity * ones, ones);
    }
}
