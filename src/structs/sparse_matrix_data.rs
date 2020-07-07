use num_traits::Num;
#[derive(Debug, Copy, Clone)]
pub struct SparseMatrixData<T: Num> {
    pub row_index: usize,
    pub col_index: usize,
    pub value: T
}

impl <T: Num + Copy> SparseMatrixData<T> {
    pub fn new(row_index: usize, col_index: usize, value: T) -> Self {
        Self {
            row_index, col_index, value
        }
    }

    pub fn transpose(&self) -> Self {
        Self {
            row_index: self.col_index,
            col_index: self.row_index,
            value: self.value
        }
    }

    pub fn transpose_mut(&mut self) {
        let temp = self.row_index;
        self.row_index = self.col_index;
        self.col_index = temp;
    }
}