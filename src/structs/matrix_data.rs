use num_traits::Num;
#[derive(Debug, Copy, Clone)]
pub struct MatrixData<T: Num> {
    row_index: usize,
    col_index: usize,
    value: T
}

impl <T: Num + Copy> MatrixData<T> {
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