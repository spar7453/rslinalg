use num_traits::Num;
use crate::structs::shape::Shape;
use crate::structs::sparse_matrix_data::SparseMatrixData;
#[derive(Debug, Clone)]
pub struct SparseMatrix<T: Num> {
    shape: Shape,
    values: Vec<SparseMatrixData<T>>
}

impl <T: Num + Copy> SparseMatrix<T> {
    pub fn new(v: &Vec<T>, row: usize, col: usize) -> Self {
        assert!(v.len() == row * col);
        let shape: Shape = Shape::new(row, col);
        let mut values: Vec<SparseMatrixData<T>> = Vec::new();
        let mut idx: usize = 0;
        for i in 0..row {
            for j in 0..col {
                let value: T = v[idx];
                if !value.is_zero() { 
                    let data = SparseMatrixData::new(i, j, value);
                    values.push(data)
                }
                idx += 1;
            }
        }
        Self {
            shape, values
        }
    }

    pub fn zeros(row: usize, col: usize) -> Self {
        let shape: Shape = Shape::new(row, col);
        let values: Vec<SparseMatrixData<T>> = Vec::new();
        Self {
            shape, values
        }
    }

    pub fn ones(row: usize, col: usize) -> Self {
        let shape: Shape = Shape::new(row, col);
        let values = (0..row).flat_map(|i| (0..col).map(move |j| SparseMatrixData::new(i,j,T::one()))).collect();
        Self {
            shape, values
        }
    }

    pub fn identity(n: usize) -> Self {
        let shape: Shape = Shape::new(n, n);
        let values: Vec<SparseMatrixData<T>> = (0..n).map(|i| SparseMatrixData::new(i, i, T::one())).collect();
        Self {
            shape, values
        }
    }

    pub fn transpose(&self) -> Self {
        Self {
            shape: self.shape.transpose(),
            values: self.values.iter().map(|data| data.transpose()).collect()
        }
    }

    pub fn transpose_mut(&mut self) {
        self.shape.transpose_mut();
        self.values.iter_mut().for_each(|mut data| data.transpose_mut());
    }
}
