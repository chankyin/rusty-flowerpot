use crate::math::{Field, FieldImpl};

pub fn comb<F: FieldImpl>(n: Field<F>, r: Field<F>) -> Field<F> {

    if n - r < r {
        return comb(n, n - r);
    }
    if r == Field::<F>::zero() {
        return Field::one();
    }
    return comb(n - Field::one(), r - Field::one()) * n / r;
}
