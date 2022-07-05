fn main() {
    let mut m = Matrix::new(2, 2);
    let mut n = Matrix::new(2, 2);
    m[(0, 0)] = 1;
    n[(0, 0)] = 2;
    let r = m + &n;
    println!("matrix: \n{}", r);
}

mod math;

use crate::math::Matrix;