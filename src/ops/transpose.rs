use crate::Matrix;

use num_traits::Zero;

impl<T: Zero + Copy, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
    pub fn transpose(&self) -> Matrix<T, COLS, ROWS> {
        let mut result: Matrix<T, COLS, ROWS> = Matrix::zeroes();

        for i in 0..ROWS {
            for j in 0..COLS {
                result[j][i] = self[i][j];
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_transpose_matrix() {
        let square = Matrix::<i32, 4, 4>::identity();

        #[rustfmt::skip]
        let nonsquare = Matrix([
            [1, 2],
        ]);

        assert_eq!(square.transpose().transpose(), square);
        assert_eq!(nonsquare.transpose().transpose(), nonsquare);
        assert_eq!(nonsquare.transpose(), Matrix([[1], [2]]))
    }
}
