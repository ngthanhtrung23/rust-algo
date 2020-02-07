// Tested: https://codeforces.com/gym/101383 - F
use std::ops::{Add, AddAssign, Mul, MulAssign};

#[derive(Clone, Copy, Debug)]
struct Modular {
	x: i64,
}

const MOD:i64 = 1_000_000_007;  // Rust does not have constant generic: https://github.com/rust-lang/rust/issues/44580
impl Modular {
	pub fn new(x: i64) -> Self {
		let mut val = x % MOD;
		if val < 0 {
			val += MOD;
		}
		Self {
			x: val,
		}
	}
}

impl Add for Modular {
	type Output = Modular;

	fn add(self, rhs: Modular) -> Self::Output {
		let mut tmp = self.x + rhs.x;
		if tmp >= MOD {
			tmp -= MOD;
		}
		return Modular::new(tmp);
	}
}
impl AddAssign for Modular {
	fn add_assign(&mut self, rhs: Self) {
		self.x += rhs.x;
		if self.x >= MOD {
			self.x -= MOD;
		}
	}
}

impl Mul for Modular {
	type Output = Modular;

	fn mul(self, rhs: Modular) -> Self::Output {
		return Modular::new((self.x * rhs.x) % MOD);
	}
}
impl MulAssign for Modular {
	fn mul_assign(&mut self, rhs: Self) {
		self.x *= rhs.x;
		self.x %= rhs.x;
	}
}

impl PartialEq for Modular {
	fn eq(&self, other: &Self) -> bool {
		self.x == other.x
	}
}

#[cfg(test)]
mod test {
	use math::modular::{Modular, MOD};

	#[test]
	fn new() {
		assert_eq!(Modular::new(1), Modular::new(MOD + 1));
		assert_eq!(Modular::new(-1), Modular::new(MOD - 1));
		assert_eq!(Modular::new(MOD * 1000 + 1), Modular::new(1));
	}

	#[test]
	fn add() {
		assert_eq!(Modular::new(1) + Modular::new(2), Modular::new(3));
		assert_eq!(Modular::new(100000000701) + Modular::new(10000000070002), Modular::new(3));
		assert_eq!(Modular::new(-1) + Modular::new(2), Modular::new(1));
		assert_eq!(Modular::new(-2) + Modular::new(1), Modular::new(- 1));
	}

	#[test]
	fn mul() {
		assert_eq!(Modular::new(1) * Modular::new(2), Modular::new(2));
		assert_eq!(Modular::new(100000000701) * Modular::new(10000000070002), Modular::new(2));
		assert_eq!(Modular::new(-1) * Modular::new(2), Modular::new(-2));
	}
}
