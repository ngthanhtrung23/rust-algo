/// Scanner class
/// Copied from https://codeforces.com/contest/1168/submission/54903799
#[derive(Default)]
pub struct Scanner {
	buffer: Vec<String>
}
impl Scanner {
	pub fn next<T: std::str::FromStr>(&mut self) -> T {
		loop {
			if let Some(token) = self.buffer.pop() {
				return token.parse().ok().expect("Failed parsing input")
			}
			let mut input = String::new();
			std::io::stdin().read_line(&mut input).expect("Failed reading input");
			self.buffer = input.split_whitespace().rev().map(String::from).collect();
		}
	}
}
