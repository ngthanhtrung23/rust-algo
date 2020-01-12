/// Returns gcd(m, n).
/// gcd(0, 0) = 0
/// gcd(x, 0) = x
/// gcd(0, x) = x
fn gcd(mut m: i64, mut n: i64) -> i64 {
	while n != 0 {
		let t = m % n;
		m = n;
		n = t;
	}
	return m;
}
 
/// Returns lcm(m, n).
fn lcm(m: i64, n: i64) -> i64 {
	return m / gcd(m, n) * n;
}
