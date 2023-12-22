use std::cmp::Ordering;

/// returns the greatest common divisor of 2 given whole numbers
pub fn gcd(mut a: usize, mut b: usize) -> usize {
    while a != b {
        match a.cmp(&b) {
            Ordering::Greater => a -= b,
            Ordering::Less => b -= a,
            Ordering::Equal => a = b,
        }
    }
    a
}
