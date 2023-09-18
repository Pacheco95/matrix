use std::ops::{Add, Div, Sub};

use num_traits::One;

use crate::Matrix;

pub struct LUDecomposition<T, const DIM: usize> {
    pub lower: Matrix<T, DIM, DIM>,
    pub upper: Matrix<T, DIM, DIM>,
    pub swaps: u32,
}

impl<T, const DIM: usize> Matrix<T, DIM, DIM>
where
    T: Default + Copy + Add<Output = T> + Sub<Output = T> + Div<Output = T> + One,
{
    /// Calculates the LU decomposition of a square matrix using the
    /// [Doolittle Algorithm](https://www.geeksforgeeks.org/doolittle-algorithm-lu-decomposition/).
    pub fn lu(&self) -> LUDecomposition<T, DIM> {
        let mut lower = Matrix::default();
        let mut upper = Matrix::default();

        let mut swaps = 0;

        for i in 0..DIM {
            for k in i..DIM {
                let mut sum = T::default();

                for j in 0..i {
                    sum = sum + lower[i][j] * upper[j][k];
                }

                upper[i][k] = self[i][k] - sum;

                swaps += 1;
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

        return LUDecomposition {
            lower,
            upper,
            swaps,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_decompose_matrix() {
        #[rustfmt::skip]
        let matrix = Matrix([
            [ 2, -1, -2],
            [-4,  6,  3],
            [-4, -2,  8],
        ]);

        let LUDecomposition { lower, upper, .. } = matrix.lu();

        #[rustfmt::skip]
        assert_eq!(lower, Matrix([
            [ 1,  0,  0],
            [-2,  1,  0],
            [-2, -1,  1],
        ]));

        #[rustfmt::skip]
        assert_eq!(upper, Matrix([
            [2, -1, -2],
            [0,  4, -1],
            [0,  0,  3],
        ]));

        assert_eq!(lower * upper, matrix);
    }
}
