use std::ops::{Add, Sub, Neg, Mul};

use crate::math::MathError;

type Number = f64;

struct Matrix {
    m: usize,
    n: usize,
    values: Vec<Number>,
}

impl Matrix {
    pub fn get(&self, i: usize, j: usize) -> Number { self.values[self.m * j + i] }

    pub fn map<F>(&self, f: F) -> Result<Matrix, MathError>
        where F: Fn(Number) -> Result<Number, MathError> {
        let mut values = Vec::with_capacity(self.m * self.n);
        for j in 0..self.n {
            for i in 0..self.m {
                values.push(f(self.get(i, j))?)
            }
        }
        Ok(Matrix { m: self.m, n: self.n, values })
    }

    pub fn bi_map<F>(mat1: &Matrix, mat2: &Matrix, f: F) -> Result<Matrix, MathError>
        where F: Fn(Number, Number) -> Result<Number, MathError> {
        if mat1.m != mat2.m || mat1.n != mat2.n {
            return Err(MathError::from("Incompatible dimensions"));
        }
        let mut values = Vec::with_capacity(mat1.m * mat1.n);
        for j in 0..mat1.n {
            for i in 0..mat1.m {
                values.push(f(mat1.get(i, j), mat2.get(i, j))?)
            }
        }
        Ok(Matrix { m: mat1.m, n: mat1.n, values })
    }
}

impl Add for &Matrix {
    type Output = Result<Matrix, MathError>;

    fn add(self, rhs: Self) -> Self::Output {
        Matrix::bi_map(self, rhs, |a, b| Ok(a + b))
    }
}

impl Sub for &Matrix {
    type Output = Result<Matrix, MathError>;

    fn sub(self, rhs: Self) -> Self::Output {
        Matrix::bi_map(self, rhs, |a, b| Ok(a - b))
    }
}

impl Neg for &Matrix {
    type Output = Result<Matrix, MathError>;

    fn neg(self) -> Self::Output {
        self.map(|a| Ok(-a))
    }
}

impl Mul for &Matrix {
    type Output = Result<Matrix, MathError>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.n != rhs.m {
            return Err(MathError::from("Incompatible dimensions"));
        }
        let mut values = Vec::with_capacity(self.m * rhs.n);
        for j in 0..rhs.n {
            for i in 0..self.m {
                let mut s = 0.0;
                for k in 0..self.n {
                    s += self.get(i, k) * rhs.get(k, j);
                }
                values.push(s);
            }
        }
        Ok(Matrix { m: self.m, n: rhs.n, values })
    }
}
