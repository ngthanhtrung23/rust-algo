// https://atcoder.jp/contests/practice2/tasks/practice2_a

pub struct Dsu {
    n: usize,
    // root node: -1 * (component size)
    // otherwise: parent
    parent_or_size: Vec<i32>,
}

impl Dsu {
    pub fn new(size: usize) -> Self {
        assert!(size < 1_000_000_000);

        Self {
            n: size,
            parent_or_size: vec![-1; size],
        }
    }

    pub fn merge(&mut self, a: usize, b: usize) -> usize {
        assert!(a < self.n);
        assert!(b < self.n);

        let (mut x, mut y) = (self.get_root(a), self.get_root(b));
        if x == y {
            return x;
        }

        // Make sure component of x is larger.
        if -self.parent_or_size[x] < -self.parent_or_size[y] {
            std::mem::swap(&mut x, &mut y);
        }

        // Parent component -> x
        self.parent_or_size[x] += self.parent_or_size[y];
        self.parent_or_size[y] = x as i32;
        x
    }

    // Return root of connected-component containing a.
    pub fn get_root(&mut self, a: usize) -> usize {
        assert!(a < self.n);
        if self.parent_or_size[a] < 0 {
            return a;
        }
        self.parent_or_size[a] = self.get_root(self.parent_or_size[a] as usize) as i32;
        self.parent_or_size[a] as usize
    }

    // Returns true if a and b belongs to same connected component.
    pub fn same(&mut self, a: usize, b: usize) -> bool {
        assert!(a < self.n);
        assert!(b < self.n);
        self.get_root(a) == self.get_root(b)
    }

    // Returns size of connected component containing a.
    pub fn size(&mut self, a: usize) -> usize {
        assert!(a < self.n);
        let root = self.get_root(a);
        -self.parent_or_size[root] as usize
    }

    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut leader_buf = vec![0; self.n];
        let mut group_size = vec![0; self.n];
        for i in 0..self.n {
            leader_buf[i] = self.get_root(i);
            group_size[leader_buf[i]] += 1;
        }
        let mut result = vec![Vec::new(); self.n];
        for i in 0..self.n {
            result[i].reserve(group_size[i]);
        }
        for i in 0..self.n {
            result[leader_buf[i]].push(i);
        }
        result.into_iter().filter(|x| !x.is_empty()).collect::<Vec<Vec<usize>>>()
    }
}

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

fn main() {
    let mut scanner = Scanner::default();

    let n = scanner.next::<usize>();
    let q = scanner.next::<usize>();

    let mut dsu = Dsu::new(n);

    for _ in 0..q {
        let typ = scanner.next::<usize>();
        let u = scanner.next::<usize>();
        let v = scanner.next::<usize>();

        if typ == 0 {
            dsu.merge(u, v);
            println!("{:?}", dsu.groups());
        } else {
            assert!(typ == 1);
            println!("{}",  if dsu.same(u, v) { 1 } else { 0 });
        }
    }
}

