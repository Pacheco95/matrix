use crate::Matrix;

impl<T, const ROWS: usize, const COLS: usize> From<[[T; COLS]; ROWS]> for Matrix<T, ROWS, COLS> {
    fn from(value: [[T; COLS]; ROWS]) -> Self {
        Self(value)
    }
}
