use std::ops::{Add, Div, Sub};

use num_traits::One;

use crate::Matrix;

impl<T, const DIM: usize> Matrix<T, DIM, DIM>
where
    T: Default + Copy + Add<Output = T> + Sub<Output = T> + Div<Output = T> + One,
{
    /// Calculates the LU decomposition of a square matrix using the
    /// [Doolittle Algorithm](https://www.geeksforgeeks.org/doolittle-algorithm-lu-decomposition/).
    pub fn lu(&self) -> (Matrix<T, DIM, DIM>, Matrix<T, DIM, DIM>) {
        let mut lower = Matrix::default();
        let mut upper = Matrix::default();

        for i in 0..DIM {
            for k in i..DIM {
                let mut sum = T::default();

                for j in 0..i {
                    sum = sum + lower[i][j] * upper[j][k];
                }

                upper[i][k] = self[i][k] - sum;
            }

            for k in i..DIM {
                if i == k {
                    lower[i][i] = T::one();
                } else {
                    let mut sum = T::default();
                    for j in 0..i {
                        sum = sum + (lower[k][j] * upper[j][i]);
                    }

                    lower[k][i] = (self[k][i] - sum) / upper[i][i];
                }
            }
        }

        return (lower, upper);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        #[rustfmt::skip]
        let matrix = Matrix([
            [ 2, -1, -2],
            [-4,  6,  3],
            [-4, -2,  8],
        ]);

        let (l, u) = matrix.lu();

        #[rustfmt::skip]
        assert_eq!(l, Matrix([
            [ 1,  0,  0],
            [-2,  1,  0],
            [-2, -1,  1],
        ]));

        #[rustfmt::skip]
        assert_eq!(u, Matrix([
            [2, -1, -2],
            [0,  4, -1],
            [0,  0,  3],
        ]));
    }
}
