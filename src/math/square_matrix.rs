use crate::math::MathError;
use crate::math::matrix::Matrix;

pub struct SquareMatrix {
    pub matrix: Matrix,
}

impl SquareMatrix {
    pub fn try_from(matrix: Matrix) -> Result<Self, MathError> {
        if matrix.m != matrix.n { Err(MathError::from("This is not a square matrix")) } else { Ok(SquareMatrix { matrix }) }
    }
}
