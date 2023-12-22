#[cfg(test)]
mod tests {
    #[test]
    fn check_gcd() {
        use crate::gcd::gcd;
        assert_eq!(gcd(164, 88), 4);
        assert_eq!(gcd(56, 24), 8);
        assert_eq!(gcd(36, 10), 2);
        assert_eq!(gcd(15, 5), 5);
        assert_eq!(gcd(899, 957), 29);
    }
}
