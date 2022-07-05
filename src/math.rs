use std::{
    fmt::Display,
    ops::{Add, Index, IndexMut},
};

pub struct Matrix {
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
        Matrix { data: v }
    }

    pub fn identity(order: usize) -> Self {
        let mut m = Self::new(order, order);
        for i in 0..order {
            m[(i, i)] = 1;
        }
        m
    }

    fn run_op(self, rhs: &Self, op: fn(i8, i8) -> i8) -> Self {
        let rows = self.data.len();
        let cols = self.data[0].len();

        let mut v = Self::new(rows, cols);
        for i in 0..rows {
            for j in 0..cols {
                v[(i, j)] = op(self[(i, j)], rhs[(i, j)]);
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
        assert_eq!(self.data.len(), rhs.data.len());
        self.run_op(rhs, |x, y| x + y)
    }
}

impl std::ops::Sub<&Matrix> for Matrix {
    type Output = Self;

    fn sub(self, rhs: &Matrix) -> Self::Output {
        assert_eq!(self.data.len(), rhs.data.len());
        self.run_op(rhs, |x, y| x - y)
    }
}
