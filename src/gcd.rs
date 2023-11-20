/// returns the greatest common divisor of 2 given whole numbers
pub fn gcd(mut a: usize, mut b: usize) -> usize {
    while a != b {
        if a > b {
            a -= b;
        } else if a < b {
            b -= a;
        }
    }
    a
}
