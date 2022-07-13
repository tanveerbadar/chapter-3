use std::{
    fmt::Display,
    ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub},
};

#[derive(PartialEq, Eq)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<Vec<i8>>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut v = Vec::<Vec<i8>>::with_capacity(rows);
        for _ in 0..rows {
            let mut r = Vec::<i8>::with_capacity(cols);
            r.resize(cols, 0);
            v.push(r);
        }
        Matrix {
            data: v,
            rows,
            cols,
        }
    }

    pub fn identity(order: usize) -> Self {
        let mut m = Self::new(order, order);
        for i in 0..order {
            m[(i, i)] = 1;
        }
        m
    }

    pub fn transpose(&self) -> Self {
        let mut m = Self::new(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                m[(i, j)] = self[(j, i)];
            }
        }
        m
    }

    fn has_quality(&self, check: Box<dyn Fn(usize, usize, i8) -> bool>) -> bool {
        if self.rows != self.cols {
            return false;
        } else {
            for i in 0..self.rows {
                for j in 0..self.cols {
                    if check(i, j, self[(i, j)]) {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn is_diagonal(&self) -> bool {
        self.has_quality(Box::new(|i, j, e| i != j && e != 0))
    }

    pub fn is_identity(&self) -> bool {
        self.has_quality(Box::new(|i, j, e| i != j && e != 0 || i == j && e != 1))
    }

    pub fn is_symmetric(&self) -> bool {
        if self.rows != self.cols {
            return false;
        } else {
            for i in 0..self.rows {
                for j in 0..self.cols {
                    if self[(i, j)] != self[(j, i)] {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn is_involutory(&self) -> bool {
        let n = 1 * self;
        let r = n * self;
        r.is_identity()
    }

    pub fn is_orthogonal(&self) -> bool {
        let n = self.transpose();
        let r = n * self;
        r.is_identity()
    }

    pub fn is_zero(&self) -> bool {
        self.has_quality(Box::new(|_, _, e| e == 0))
    }

    pub fn trace(&self) -> i8 {
        assert_eq!(self.rows, self.cols);
        let mut sum = 0;
        for i in 0..self.rows {
            sum += self[(i, i)];
        }
        sum
    }

    fn run_op(&self, rhs: &Self, op: fn(i8, i8) -> i8) -> Self {
        assert_eq!(self.rows, rhs.rows);
        assert_eq!(self.cols, rhs.cols);
        let rows = self.rows;
        let cols = self.cols;

        let mut v = Self::new(rows, cols);
        for i in 0..rows {
            for j in 0..cols {
                v[(i, j)] = op(self[(i, j)], rhs[(i, j)]);
            }
        }
        v
    }

    fn run_uniop(&self, op: Box<dyn Fn(i8) -> i8>) -> Self {
        let rows = self.rows;
        let cols = self.cols;

        let mut v = Self::new(rows, cols);
        for i in 0..rows {
            for j in 0..cols {
                v[(i, j)] = op(self[(i, j)]);
            }
        }
        v
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = i8;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0][index.1]
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in self.data.iter() {
            writeln!(f, "{:?}", r)?;
        }
        Ok(())
    }
}

impl Add<&Matrix> for Matrix {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        self.run_op(rhs, |x, y| x + y)
    }
}

impl Sub<&Matrix> for Matrix {
    type Output = Self;

    fn sub(self, rhs: &Matrix) -> Self::Output {
        self.run_op(rhs, |x, y| x - y)
    }
}

impl Mul<&Matrix> for Matrix {
    type Output = Self;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        assert_eq!(self.cols, rhs.rows);
        let mut m = Matrix::new(self.rows, rhs.cols);
        for i in 0..self.rows {
            for j in 0..rhs.cols {
                for k in 0..self.cols {
                    m[(i, j)] += self[(i, k)] * rhs[(k, j)];
                }
            }
        }
        m
    }
}

impl Add<i8> for Matrix {
    type Output = Self;

    fn add(self, rhs: i8) -> Self::Output {
        self.run_uniop(Box::new(move |x| x + rhs))
    }
}

impl Add<&Matrix> for i8 {
    type Output = Matrix;

    fn add(self, rhs: &Matrix) -> Self::Output {
        rhs.run_uniop(Box::new(move |x| x + self))
    }
}

impl Sub<i8> for Matrix {
    type Output = Self;

    fn sub(self, rhs: i8) -> Self::Output {
        self.run_uniop(Box::new(move |x| x - rhs))
    }
}

impl Mul<i8> for Matrix {
    type Output = Self;

    fn mul(self, rhs: i8) -> Self::Output {
        self.run_uniop(Box::new(move |x| x * rhs))
    }
}

impl Mul<&Matrix> for i8 {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        rhs.run_uniop(Box::new(move |x| x * self))
    }
}

impl Div<i8> for Matrix {
    type Output = Self;

    fn div(self, rhs: i8) -> Self::Output {
        self.run_uniop(Box::new(move |x| x / rhs))
    }
}

impl Neg for Matrix {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.run_uniop(Box::new(move |x| -x))
    }
}
