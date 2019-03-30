use std::ops::{Add, Mul, Neg, Sub};

use crate::math::MathError;

type Number = f64;

#[derive(Debug)]
pub struct Matrix {
    pub m: usize,
    pub n: usize,
    values: Vec<Number>,
}

impl Matrix {
    pub fn m(&self) -> usize { self.m }
    pub fn n(&self) -> usize { self.n }
    pub fn get(&self, i: usize, j: usize) -> Number { self.values[self.m * j + i] }

    pub fn create<F>(m: usize, n: usize, f: F) -> Self
        where F: Fn(usize, usize) -> Number {
        Self::try_create(m, n, |i, j| Ok(f(i, j))).unwrap()
    }

    pub fn try_create<F>(m: usize, n: usize, f: F) -> Result<Self, MathError>
        where F: Fn(usize, usize) -> Result<Number, MathError> {
        let mut values = Vec::with_capacity(m * n);
        for j in 0..n {
            for i in 0..m {
                values.push(f(i, j)?)
            }
        }
        Ok(Matrix { m, n, values })
    }

    fn map<F>(&self, f: F) -> Result<Matrix, MathError>
        where F: Fn(Number) -> Result<Number, MathError> {
        Matrix::try_create(self.m, self.n, |i, j| f(self.get(i, j)))
    }
    fn bi_map<F>(mat1: &Matrix, mat2: &Matrix, f: F) -> Result<Matrix, MathError>
        where F: Fn(Number, Number) -> Result<Number, MathError> {
        if mat1.m != mat2.m || mat1.n != mat2.n {
            return Err(MathError::from("Incompatible dimensions"));
        }
        Matrix::try_create(mat1.m, mat1.n, |i, j| f(mat1.get(i, j), mat2.get(i, j)))
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.m != other.m || self.n != other.n { return false; }
        for j in 0..self.n {
            for i in 0..self.m {
                if self.get(i, j) != other.get(i, j) { return false; }
            }
        }
        true
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
        Matrix::try_create(self.m(), rhs.n(), |i, j| {
            let mut s = 0.0;
            for k in 0..self.n() {
                s += self.get(i, k) * rhs.get(k, j);
            }
            Ok(s)
        })
    }
}

#[cfg(test)]
mod test {
    extern crate conv;

    use conv::ConvUtil;

    use crate::math::matrix::{Matrix, Number};

    macro_rules! assert_mat_eq {
        ($actual: expr, $expect: expr) => {
            let actual = $actual.unwrap();
            let expect = $expect;
            assert_eq!(actual.m(), expect.m());
            assert_eq!(actual.n(), expect.n());
            for i in 0..actual.m() {
                for j in 0..actual.n() {
                    assert_eq!(actual.get(i, j), expect.get(i, j));
                }
            }
        };
    }

    fn as_number(t: usize) -> Number {
        t.value_as::<Number>().unwrap()
    }

    #[test]
    pub fn test_add() {
        let mat1 = Matrix::create(3, 2, |i, j| as_number(10 * i + j));
        let mat2 = Matrix::create(3, 2, |i, j| as_number(i + 10 * j));
        assert_mat_eq!(&mat1 + &mat2, Matrix::create(3, 2, |i, j| as_number(11 * (i + j))));
    }

    #[test]
    pub fn test_sub() {
        let mat1 = Matrix::create(3, 2, |i, j| as_number(10 * i + j));
        let mat2 = Matrix::create(3, 2, |i, j| as_number(i + 10 * j));
        assert_mat_eq!(&mat1 - &mat2, Matrix::create(3, 2,
            |i, j| (as_number(i) - as_number(j)) * 9.0));
    }

    #[test]
    pub fn test_mul() {
        let mat1 = Matrix::create(3, 2, |i, j| as_number(10 * i + j));
        let mat2 = Matrix::create(2, 3, |i, j| as_number(i + 10 * j));
        dbg!(&mat1 * &mat2);
    }
}
