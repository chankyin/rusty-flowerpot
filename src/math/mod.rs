use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};

//pub mod number_theory;
pub mod matrix;
pub mod square_matrix;

#[derive(Debug)]
pub struct MathError {
    message: String,
}

impl Display for MathError where Self: Sized {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message.as_str())
    }
}

impl Error for MathError {}

impl From<String> for MathError {
    fn from(message: String) -> Self { MathError { message } }
}

impl From<&str> for MathError {
    fn from(message: &str) -> Self { MathError { message: message.to_owned() } }
}

pub struct Field<F: FieldImpl> { pub val: F }

impl<F: FieldImpl> Field<F> {
    fn one() -> Self { Self { val: F::one() } }
    fn zero() -> Self { Self { val: F::zero() } }
}

impl<F: FieldImpl> Clone for Field<F> {
    fn clone(&self) -> Self { Self { val: self.val } }
}

impl<F: FieldImpl> Copy for Field<F> {}

impl<F: FieldImpl> Add for Field<F> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self { Field { val: self.val.field_add(rhs.val) } }
}

impl<F: FieldImpl> Sub for Field<F> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self { Field { val: self.val.field_sub(rhs.val) } }
}

impl<F: FieldImpl> Mul for Field<F> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self { Field { val: self.val.field_mul(rhs.val) } }
}

impl<F: FieldImpl> Div for Field<F> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self { Field { val: self.val.field_div(rhs.val) } }
}

impl<F: FieldImpl> PartialOrd for Field<F> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { self.val.field_cmp(&other.val) }
}

impl<F: FieldImpl> PartialEq for Field<F> {
    fn eq(&self, other: &Self) -> bool { self.val.field_eq(&other.val) }
}

pub trait FieldImpl: Copy {
    fn field_add(&self, that: Self) -> Self;
    fn field_sub(&self, that: Self) -> Self;
    fn field_mul(&self, that: Self) -> Self;
    fn field_div(&self, that: Self) -> Self;
    fn field_cmp(&self, that: &Self) -> Option<Ordering>;
    fn field_eq(&self, that: &Self) -> bool;
    fn additive_identity() -> Self;
    fn zero() -> Self { Self::additive_identity() }
    fn multiplicative_identity() -> Self;
    fn one() -> Self { Self::multiplicative_identity() }
}

pub type Number = f64;

impl FieldImpl for Number {
    fn field_add(&self, that: Self) -> Self { self + that }
    fn field_sub(&self, that: Self) -> Self { self - that }
    fn field_mul(&self, that: Self) -> Self { self * that }
    fn field_div(&self, that: Self) -> Self { self / that }
    fn field_cmp(&self, that: &Self) -> Option<Ordering> { self.partial_cmp(that) }
    fn field_eq(&self, that: &Self) -> bool { self.eq(that) }
    fn additive_identity() -> Self { 0f64 }
    fn multiplicative_identity() -> Self { 1f64 }
}
