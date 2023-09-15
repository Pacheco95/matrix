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
