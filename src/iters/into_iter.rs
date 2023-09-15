use crate::Matrix;

impl<'a, T, const ROWS: usize, const COLS: usize> IntoIterator for &'a Matrix<T, ROWS, COLS> {
    type Item = (usize, usize, &'a T);

    type IntoIter = MatrixIterator<'a, T, ROWS, COLS>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            matrix: self,
            index: 0,
        }
    }
}

pub struct MatrixIterator<'a, T, const ROWS: usize, const COLS: usize> {
    matrix: &'a Matrix<T, ROWS, COLS>,
    index: usize,
}

impl<'a, T, const ROWS: usize, const COLS: usize> Iterator for MatrixIterator<'a, T, ROWS, COLS> {
    type Item = (usize, usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == ROWS * COLS {
            return None;
        }

        self.index += 1;

        Some((0, 0, &self.matrix[0][0]))
    }
}
