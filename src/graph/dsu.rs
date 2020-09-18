// Translated to Rust from https://codeforces.com/blog/entry/82400
//
// Notes:
// - Index from 0
//
// Usage:
//   Dsu dsu = Dsu::new(n);
//   dsu.same(i, j)
//   dsu.merge(i, j)
//   dsu.size(i)
//   dsu.groups()
//
// Tested:
// - https://atcoder.jp/contests/practice2/tasks/practice2_a

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dsu() {
        // Init: Graph with 4 vertices and 0 edges.
        let mut dsu = Dsu::new(4);
        for i in 0..4 {
            assert!(dsu.same(i, i));
            assert_eq!(dsu.size(i), 1);
        }
        assert!(!dsu.same(0, 1));
        assert_eq!(dsu.groups().len(), 4);

        // Connect 0-1
        dsu.merge(0, 1);
        assert!(dsu.same(0, 1));
        assert_eq!(dsu.groups().len(), 3);
        for i in 0..2 {
            assert_eq!(dsu.size(i), 2);
        }

        // Connect 2-3
        dsu.merge(2, 3);
        assert!(dsu.same(2, 3));
        assert_eq!(dsu.groups().len(), 2);
        for i in 0..4 {
            assert_eq!(dsu.size(i), 2);
        }

        // Connect 1-2
        dsu.merge(1, 2);
        for i in 0..4 {
            assert_eq!(dsu.size(i), 4);
            for j in i+1..4 {
                assert!(dsu.same(i, j));
            }
        }
        assert_eq!(dsu.groups().len(), 1);
    }
}
