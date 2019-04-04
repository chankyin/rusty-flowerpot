use std::cmp::Ordering;

use crate::math::FieldImpl;

pub mod gcd;
pub mod comb;

pub struct ModRing {
    pub k: u32,
    pub n: u32,
}

impl FieldImpl for ModRing {
    fn field_add(&self, that: Self) -> Self { Self { k: self.k + that.k, n: self.n } }
    fn field_sub(&self, that: Self) -> Self { unimplemented!() }
    fn field_mul(&self, that: Self) -> Self { unimplemented!() }
    fn field_div(&self, that: Self) -> Self { unimplemented!() }
    fn field_cmp(&self, that: &Self) -> Option<Ordering> { unimplemented!() }
    fn field_eq(&self, that: &Self) -> bool { unimplemented!() }
    fn additive_identity() -> Self { unimplemented!() }
    fn multiplicative_identity() -> Self {  }
}
