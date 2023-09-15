use std::ops::{Add, Mul, Sub};

use num_traits::One;

use crate::Matrix;

impl<T: One> Matrix<T, 0, 0> {
    #[inline(always)]
    pub fn det(&self) -> T {
        T::one()
    }
}

impl<T: One + Copy> Matrix<T, 1, 1> {
    #[inline(always)]
    pub fn det(&self) -> T {
        self[0][0]
    }
}

impl<T: Copy + Sub<Output = T> + Mul<Output = T>> Matrix<T, 2, 2> {
    #[inline(always)]
    pub fn det(&self) -> T {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }
}

impl<T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>> Matrix<T, 3, 3> {
    #[inline(always)]
    pub fn det(&self) -> T {
        #[rustfmt::skip]
        let [
            [a, b, c],
            [d, e, f],
            [g, h, i]
        ] = self.0;

        (a * e * i) + (b * f * g) + (c * d * h) - (c * e * g) - (b * d * i) - (a * f * h)
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
}
