// https://judge.yosupo.jp/problem/associative_array

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

use std::collections::HashMap;

fn main() {
    let mut scanner = Scanner::default();

    let mut mp: HashMap<i64, i64> = HashMap::new();

    let q = scanner.next::<i32>();
    for _ in 0..q {
        let typ = scanner.next::<i8>();
        let k = scanner.next::<i64>();
        if typ == 0 {
            let v = scanner.next::<i64>();
            mp.insert(k, v);
        } else {
            let v: i64 = match mp.get(&k) {
                None => 0,
                Some(&x) => x,
            };
            println!("{}", v);
        }
    }
}

