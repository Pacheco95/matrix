pub mod converters;
pub mod iters;
pub mod ops;

use num_traits::{One, Zero};

#[derive(Debug, Clone, Copy)]
pub struct Matrix<T, const ROWS: usize, const COLS: usize>(pub [[T; COLS]; ROWS]);

impl<T, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
    #[inline]
    pub fn dim(&self) -> (usize, usize) {
        (ROWS, COLS)
    }

    #[inline]
    pub fn is_row_vector(&self) -> bool {
        ROWS == 1
    }

    #[inline]
    pub fn is_column_vector(&self) -> bool {
        COLS == 1
    }

    #[inline]
    pub fn elements(&self) -> impl Iterator<Item = &T> {
        self.0.iter().flat_map(|line| line.iter())
    }
}

impl<T, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS>
where
    T: Zero + Copy,
{
    pub fn zeroes() -> Self {
        Self([[T::zero(); COLS]; ROWS])
    }
}

impl<T, const DIM: usize> Matrix<T, DIM, DIM>
where
    T: One + Copy,
{
    pub fn ones() -> Self {
        Self([[T::one(); DIM]; DIM])
    }
}

impl<T, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS>
where
    T: Default + Copy,
{
    pub fn default() -> Self {
        Self([[T::default(); COLS]; ROWS])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use itertools::Itertools;
    use std::mem::{size_of, size_of_val};

    type ElementType = i32;
    type Mat<const R: usize, const C: usize> = Matrix<ElementType, R, C>;

    #[test]
    fn it_should_create_vectors() {
        assert!(Mat::<1, 1>::default().is_row_vector());
        assert!(Mat::<1, 1>::default().is_column_vector());

        assert!(!Mat::<3, 1>::default().is_row_vector());
        assert!(Mat::<3, 1>::default().is_column_vector());

        assert!(Mat::<1, 3>::default().is_row_vector());
        assert!(!Mat::<1, 3>::default().is_column_vector());

        assert!(!Mat::<3, 3>::default().is_row_vector());
        assert!(!Mat::<3, 3>::default().is_column_vector());
    }

    #[test]
    fn it_should_return_correct_dimensions() {
        assert_eq!(Mat::<1, 1>::default().dim(), (1, 1));
        assert_eq!(Mat::<1, 2>::default().dim(), (1, 2));
        assert_eq!(Mat::<2, 1>::default().dim(), (2, 1));
    }

    #[test]
    fn it_should_create_matrix_with_same_size_than_array() {
        const ROWS: usize = 2;
        const COLS: usize = 3;

        type T = isize;
        type Mat = Matrix<T, ROWS, COLS>;
        type RawMat = [[T; COLS]; ROWS];

        assert!(size_of::<Mat>() == size_of::<RawMat>());

        let raw_matrix: RawMat = [[1, 2, 3], [4, 5, 6]];
        let mat: Mat = raw_matrix.try_into().expect("Should never fail");

        assert!(size_of_val(&mat) == size_of_val(&raw_matrix));
    }

    #[test]
    fn test_elements_iterator() {
        let matrix = Mat::<3, 3>::identity();

        let actual = matrix.elements().collect_vec();

        #[rustfmt::skip]
        let expected = vec![
            &1, &0, &0,
            &0, &1, &0,
            &0, &0, &1,
        ];

        assert_eq!(actual, expected);
    }
}
