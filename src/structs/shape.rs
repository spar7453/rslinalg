#[derive(Debug, Copy, Clone)]
pub struct Shape {
    row: usize,
    col: usize
}

impl Shape {
    pub fn new(row: usize, col: usize) -> Self {
        Self {
            row, col
        }
    }

    pub fn transpose(&self) -> Self {
        Self {
            row: self.col,
            col: self.row
        }
    }

    pub fn transpose_mut(&mut self) {
        let temp = self.row;
        self.row = self.col;
        self.col = temp;
    }
}