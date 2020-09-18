// https://judge.yosupo.jp/problem/static_range_sum

#[derive(Default)]
struct Scanner {
	buffer: Vec<String>
}
impl Scanner {
	fn next<T: std::str::FromStr>(&mut self) -> T {
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

/// Read n space-separated i64 into a Vec.
fn read_vec<T: std::str::FromStr>(scanner: &mut Scanner, n: usize) -> Vec<T> {
    (0..n).map(|_| scanner.next::<T>()).collect()
}

/// Returns prefix sum of a Vec.
fn prefix_sum(a: &Vec<i64>) -> Vec<i64> {
	return a.iter().scan(0, |sum, &x| {
		*sum += x;
		Some(*sum)
	}).collect::<Vec<_>>();
}

fn get_sum(s: &Vec<i64>, l: usize, r: usize) -> i64 {
    if l == 0 {
        return s[r];
    }
    return s[r] - s[l - 1];
}

fn main() {
    let mut scanner = Scanner::default();

    let n = scanner.next::<usize>();
    let q = scanner.next::<i32>();

    let a = read_vec(&mut scanner, n);
    let s = prefix_sum(&a);

    for _ in 0..q {
        let l = scanner.next::<usize>();
        let r = scanner.next::<usize>() - 1;

        println!("{}", get_sum(&s, l, r));
    }
}

