pub fn gcd(mut a: usize, mut b: usize) -> usize {
    while a != b {
        if a == b {
            break;
        } else if a > b {
            a -= b;
        } else if a < b {
            b -= a;
        }
    }
    a
}
