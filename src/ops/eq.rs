use std::ops::{Index, IndexMut};

use crate::Matrix;

impl<T, const ROWS: usize, const COLS: usize> Index<usize> for Matrix<T, ROWS, COLS> {
    type Output = [T; COLS];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T, const ROWS: usize, const COLS: usize> IndexMut<usize> for Matrix<T, ROWS, COLS> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: PartialEq, const ROWS: usize, const COLS: usize> PartialEq<[[T; COLS]; ROWS]>
    for Matrix<T, ROWS, COLS>
{
    fn eq(&self, raw: &[[T; COLS]; ROWS]) -> bool {
        for row in 0..ROWS {
            for col in 0..COLS {
                if self[row][col] != raw[row][col] {
                    return false;
                }
            }
        }

        true
    }
}

impl<T: PartialEq, const ROWS: usize, const COLS: usize> PartialEq<Matrix<T, ROWS, COLS>>
    for Matrix<T, ROWS, COLS>
{
    fn eq(&self, raw: &Self) -> bool {
        self == &raw.0
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    type ElementType = i32;
    type Mat<const R: usize, const C: usize> = Matrix<ElementType, R, C>;

    #[test]
    fn it_should_create_matrices_correctly() -> Result<(), Box<dyn Error>> {
        assert_eq!(Mat::<1, 1>::default(), [[0]]);
        assert_eq!(Mat::<1, 2>::default(), [[0, 0]]);
        assert_eq!(Mat::<2, 1>::default(), [[0], [0]]);
        assert_eq!(Mat::<2, 2>::default(), [[0, 0], [0, 0]]);

        let m = Matrix([[1, 2], [3, 4], [5, 6]]);

        assert_eq!(m[0][0], 1);
        assert_eq!(m[0][1], 2);
        assert_eq!(m[1][0], 3);
        assert_eq!(m[1][1], 4);
        assert_eq!(m[2][0], 5);
        assert_eq!(m[2][1], 6);

        Ok(())
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn it_should_panic_index_out_of_bounds() {
        Mat::<1, 1>::ones()[2][2];
    }
}
