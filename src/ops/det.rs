use std::ops::{Add, Div, Mul, Neg, Sub};

use num_traits::One;

use crate::Matrix;

use super::lu::LUDecomposition;

#[inline(always)]
fn det0<T: One>(_mat: &Matrix<T, 0, 0>) -> T {
    T::one()
}

#[inline(always)]
fn det1<T: Copy>(mat: &Matrix<T, 1, 1>) -> T {
    mat[0][0]
}

#[inline(always)]
fn det2<T: Copy + Sub<Output = T> + Mul<Output = T>>(mat: &Matrix<T, 2, 2>) -> T {
    mat[0][0] * mat[1][1] - mat[0][1] * mat[1][0]
}

#[inline(always)]
fn det3<T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>>(mat: &Matrix<T, 3, 3>) -> T {
    #[rustfmt::skip]
        let [
            [a, b, c],
            [d, e, f],
            [g, h, i]
        ] = mat.0;

    (a * e * i) + (b * f * g) + (c * d * h) - (c * e * g) - (b * d * i) - (a * f * h)
}

impl<T, const DIM: usize> Matrix<T, DIM, DIM>
where
    T: Default + Copy + Add<Output = T> + Sub<Output = T> + Div<Output = T> + Neg<Output = T> + One,
{
    #[inline(always)]
    pub fn det(&self) -> T {
        return match DIM {
            0usize => unsafe {
                let cast = self as *const Matrix<T, DIM, DIM> as *const Matrix<T, 0, 0>;
                det0(&*cast)
            },
            1usize => unsafe {
                let cast = self as *const Matrix<T, DIM, DIM> as *const Matrix<T, 1, 1>;
                det1(&*cast)
            },
            2usize => unsafe {
                let cast = self as *const Matrix<T, DIM, DIM> as *const Matrix<T, 2, 2>;
                det2(&*cast)
            },
            3usize => unsafe {
                let cast = self as *const Matrix<T, DIM, DIM> as *const Matrix<T, 3, 3>;
                det3(&*cast)
            },
            _ => {
                // TODO check it out https://www.rustexplorer.com/b/ix3mac
                // https://discordapp.com/channels/273534239310479360/273541522815713281/1152424473669161000

                let LUDecomposition {
                    lower,
                    upper,
                    swaps,
                } = self.lu();

                let mut lower_det = T::one();
                let mut upper_det = T::one();

                for i in 0..DIM {
                    lower_det = lower_det * lower[i][i];
                    upper_det = upper_det * upper[i][i];
                }

                let signal = if swaps % 2 == 0 { T::one() } else { -T::one() };

                signal * lower_det * upper_det
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_calculate_determinant_for_0x0_matrix() {
        type Mat = Matrix<i32, 0, 0>;
        assert_eq!(Mat::default().det(), 1)
    }

    #[test]
    fn it_should_calculate_determinant_for_1x1_matrix() {
        assert_eq!(Matrix([[100]]).det(), 100)
    }

    #[test]
    fn it_should_calculate_determinant_for_2x2_matrix() {
        #[rustfmt::skip]
        let mat = Matrix([
            [1, 2],
            [3, 4]
        ]);

        assert_eq!(mat.det(), -2)
    }

    #[test]
    fn it_should_calculate_determinant_for_3x3_matrix() {
        #[rustfmt::skip]
        let mat = Matrix([
            [-7, -10,  4],
            [ 3,  -9,  2],
            [ 7,   1,  2],
        ]);

        assert_eq!(mat.det(), 324)
    }

    #[test]
    fn it_should_calculate_determinant_for_4x4_matrix() {
        #[rustfmt::skip]
        let mat = Matrix([
            [1., 2., 1., 2.],
            [-1., 0., 2., -2.],
            [-2., 2., 0., 0.],
            [-1., 1., -1., 1.],
        ]);

        assert!(mat.det() - (-6.) < 0.001);

        #[rustfmt::skip]
        let mat: Matrix<f32, 4, 4> = Matrix([
            [1., 0., 4., -6.],
            [2., 5., 0., 3.],
            [-1., 2., 3., 5.],
            [2., 1., -2., 3.],
        ]);

        assert_eq!(mat.det(), 318.0)
    }
}
