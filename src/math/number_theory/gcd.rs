use std::ops::Rem;

pub fn gcd(mut a: usize, mut b: usize) -> usize{
    while b != 0 {
        let c = b;
        b = a.rem(b);
        a = c;
    }
    a
}
