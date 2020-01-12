/// Returns gcd(m, n).
/// gcd(0, 0) = 0
/// gcd(x, 0) = x
/// gcd(0, x) = x
/// For negative numbers, return gcd(|m|, |n|).
fn gcd(mut m: i64, mut n: i64) -> i64 {
	while n != 0 {
		m %= n;
		std::mem::swap(&mut m, &mut n);
	}
	return m.abs();
}
 
/// Returns lcm(m, n).
/// Does not work with negative numbers.
fn lcm(m: i64, n: i64) -> i64 {
    if m == 0 || n == 0 {
        return 0;
    }
	return m / gcd(m, n) * n;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd(0, 10), 10);
        assert_eq!(gcd(10, 0), 10);
        assert_eq!(gcd(6, 12), 6);
        assert_eq!(gcd(6, 6), 6);
        assert_eq!(gcd(16, 28), 4);
        assert_eq!(gcd(7, 11), 1);

        // Negative numbers.
        assert_eq!(gcd(0, -10), 10);
        assert_eq!(gcd(-10, 0), 10);
        assert_eq!(gcd(-6, -12), 6);
        assert_eq!(gcd(-6, 12), 6);
        assert_eq!(gcd(6, -12), 6);
        assert_eq!(gcd(-6, -6), 6);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(0, 0), 0);
        assert_eq!(lcm(0, 10), 0);
        assert_eq!(lcm(10, 0), 0);
        assert_eq!(lcm(6, 12), 12);
        assert_eq!(lcm(16, 28), 16 * 7);
        assert_eq!(lcm(7, 11), 77);
    }
}
