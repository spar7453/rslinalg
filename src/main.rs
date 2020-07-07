pub mod structs;
use structs::shape::*;
use structs::sparse_matrix_data::*;
use structs::sparse_matrix::*;
use structs::csr_matrix::*;
fn main() {
   // let shape = Shape::new(1,2);
   // println!("{:?}", shape);
   // let shape = shape.transpose();
   // println!("{:?}", shape.transpose());
   // println!("{:?}", shape);

   // println!("-===================mut!");
   // let mut shape = Shape::new(1,2);
   // println!("{:?}", shape);
   // println!("{:?}", shape.transpose_mut());
   // println!("{:?}", shape);

    let v = vec![1,2,0,5,0,8,0,0,9];

    let mat = SparseMatrix::new(&v, 3, 3);
    println!("vector insert = {:?}", v);
    println!("mat = {:#?}", mat);
    //println!("mat.t = {:#?}", mat.transpose());

    let x: i32 = 1;
    let r = &&x;
    let zz: i32 = x.pow(1);
    let z = r.is_negative();

    //let zeros: SparseMatrix<i32> = SparseMatrix::zeros(3, 2);
    //let ones: SparseMatrix<i32> = SparseMatrix::ones(3, 2);
    //let identity: SparseMatrix<i32> = SparseMatrix::identity(3);

    //println!("zeros = {:#?}", zeros);
    //println!("ones = {:#?}", ones);
    //println!("identity = {:#?}", identity);

    ////let v = vec![0,0,0,0,1,5,8,0,0,0,0,0,3,0,0,0,6,0,0,1];
    ////let mat = CsrMatrix::new(&v, 4, 5);
    //let v = vec![1,2,0,0,1,1,2,0,3];
    //let v = vec![10,20,0,0,0,0,0,30,0,40,0,0,0,0,50,60,70,0,0,0,0,0,0,80];
    //let mat = CsrMatrix::new(&v, 4, 6);
    //println!("csr = {:#?}", mat);
}