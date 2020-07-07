use num_traits::Num;
use crate::structs::shape::Shape;

#[derive(Debug, Clone)]
pub struct CsrMatrix<T: Num> {
    shape: Shape,
    values: Vec<T>,
    col_index: Vec<usize>,
    row_ptr: Vec<usize>
}

impl <T: Num + Copy> CsrMatrix<T> {
    pub fn new(v: &Vec<T>, row: usize, col: usize) -> Self {
        assert!(v.len() == row * col);
        let shape: Shape = Shape::new(row, col);
        let mut values: Vec<T> = Vec::new();
        let mut col_index: Vec<usize> = Vec::new();
        let mut row_ptr: Vec<usize> = vec!(0);
        let mut idx: usize = 0;
        let mut non_zero: usize = 0;

        for _ in 0..row {
            for j in 0..col {
                let value: T = v[idx];
                if !value.is_zero() { 
                    values.push(value);
                    col_index.push(j);
                    non_zero += 1;
                }
                idx += 1;
            }
            row_ptr.push(non_zero);
        }
        Self {
            shape, values, col_index, row_ptr
        }
    }

    pub fn zeros(row: usize, col: usize) -> Self {
        let shape: Shape = Shape::new(row, col);
        let values: Vec<T> = Vec::new();
        let col_index: Vec<usize> = Vec::new();
        let row_ptr: Vec<usize> = Vec::new();
        Self {
            shape, values, col_index, row_ptr
        }
    }

    pub fn ones(row: usize, col: usize) -> Self {
        let shape: Shape = Shape::new(row, col);
        let values: Vec<T> = vec![T::one(); row * col];
        let col_index: Vec<usize> = (0..row).flat_map(|_| (0..col)).collect();
        let row_ptr: Vec<usize> = (0..row).map(|x| x * col).collect();
        Self {
            shape, values, col_index, row_ptr
        }
    }

    pub fn identity(n: usize) -> Self {
        let interval: usize = n + 1;
        let shape: Shape = Shape::new(n, n);
        let values: Vec<T> = vec![T::one(); n];
        let col_index: Vec<usize> = (0..n).collect();
        let row_ptr: Vec<usize> = (0..n).map(|x| x * interval).collect();
        Self {
            shape, values, col_index, row_ptr
        }
    }

    //pub fn transpose(&self) -> Self {
    //    Self {
    //        shape: self.shape.transpose(),
    //        values: self.values.iter().map(|data| data.transpose()).collect()
    //    }
    //}

    //pub fn transpose_mut(&mut self) {
    //    self.shape.transpose_mut();
    //    self.values.iter_mut().for_each(|mut data| data.transpose_mut());
    //}
}