pub mod structs;
use structs::shape::*;
use structs::sparse_matrix_data::*;
use structs::sparse_matrix::*;
fn main() {
    let shape = Shape::new(1,2);
    println!("{:?}", shape);
    let shape = shape.transpose();
    println!("{:?}", shape.transpose());
    println!("{:?}", shape);

    println!("-===================mut!");
    let mut shape = Shape::new(1,2);
    println!("{:?}", shape);
    println!("{:?}", shape.transpose_mut());
    println!("{:?}", shape);

    let v = vec![1,1,0,0,0,6];

    let mat = SparseMatrix::new(&v, 3, 2);
    println!("vector insert = {:?}", v);
    println!("mat = {:#?}", mat);
    println!("mat.t = {:?}", mat.transpose());

    let zeros: SparseMatrix<i32> = SparseMatrix::zeros(3, 2);
    let ones: SparseMatrix<i32> = SparseMatrix::ones(3, 2);
    let identity: SparseMatrix<i32> = SparseMatrix::identity(3);

    println!("zeros = {:#?}", zeros);
    println!("ones = {:#?}", ones);
    println!("identity = {:#?}", identity);

}