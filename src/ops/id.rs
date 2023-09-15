use num_traits::{One, Zero};

use crate::Matrix;

impl<T, const DIM: usize> Matrix<T, DIM, DIM>
where
    T: Zero + One + Copy,
{
    pub fn identity() -> Self {
        let mut matrix: Matrix<T, DIM, DIM> = Self::zeroes();

        for row in 0..DIM {
            for col in 0..DIM {
                if row == col {
                    matrix[row][col] = T::one();
                }
            }
        }

        matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_shoud_create_identity_matrix() {
        let matrix = Matrix::<f32, 3, 3>::identity();

        for (row, col, el) in &matrix {
            if row == col {
                assert_eq!(el, &1f32);
            } else {
                assert_eq!(el, &0f32);
            }
        }
    }
}
