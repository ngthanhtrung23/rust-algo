// Tested: https://codeforces.com/gym/101383 - C

use graph::Graph;
use std::borrow::BorrowMut;
use std::cmp::min;

// Build strongly connected components.
struct StronglyConnectedComponents {
	sccs: Vec<Vec<usize>>,
	// The strongly connected components.
	num: Vec<i32>,
	low: Vec<i32>,
	current: Vec<i32>,
	st: Vec<usize>,
	counter: i32,
}

impl StronglyConnectedComponents {
	pub fn new(graph: &Graph) -> Self {
		let v = graph.n_vertex;
		Self {
			sccs: vec![],
			num: vec![-1; v],
			low: vec![0; v],
			current: vec![0; v],
			st: vec![],
			counter: 0,
		}
	}
}

impl Graph {
	pub fn build_scc(&self) -> StronglyConnectedComponents {
		let mut result = StronglyConnectedComponents::new(self);
		for u in 0..self.n_vertex {
			if result.num[u] < 0 {
				self.dfs(u, result.borrow_mut());
			}
		}
		result
	}

	fn dfs(&self, u: usize, result: &mut StronglyConnectedComponents) {
		result.low[u] = result.counter;
		result.num[u] = result.counter;
		result.counter += 1;

		result.st.push(u);
		result.current[u] = 1;

		for &v in self.adjs[u].iter() {
			if result.num[v] == -1 {
				self.dfs(v, result);
			}
			if result.current[v] != 0 {
				result.low[u] = min(result.low[u], result.low[v]);
			}
		}

		if result.low[u] == result.num[u] {
			let mut scc = vec![];
			while let Some(v) = result.st.pop() {
				result.current[v] = 0;
				scc.push(v);
				if u == v {
					break;
				}
			}
			result.sccs.push(scc);
		}
	}
}

#[cfg(test)]
mod test {
	use graph::Graph;

	#[test]
	fn build_scc() {
		let mut graph = Graph::new(6);

		graph.add_directed_edge(0, 1);
		graph.add_directed_edge(1, 2);
		graph.add_directed_edge(2, 0);

		graph.add_directed_edge(4, 5);
		graph.add_directed_edge(5, 4);

		let mut sccs = graph.build_scc();
		assert_eq!(sccs.sccs.len(), 3);

		// Sort each scc, so that we can compare.
		for scc in sccs.sccs.iter_mut() {
			scc.sort();
		}
		println!("{:?}", sccs.sccs);
		assert!(sccs.sccs.iter().any(|scc| *scc == vec![0, 1, 2]));
		assert!(sccs.sccs.iter().any(|scc| *scc == vec![3]));
		assert!(sccs.sccs.iter().any(|scc| *scc == vec![4, 5]));
	}

	#[test]
	fn build_scc_empty_graph() {
		let graph = Graph::new(0);
		let sccs = graph.build_scc();
		assert_eq!(sccs.sccs.len(), 0);
	}

	#[test]
	fn build_scc_no_edge() {
		let n = 100;
		let graph = Graph::new(n);
		let sccs = graph.build_scc();
		assert_eq!(sccs.sccs.len(), n);

		for i in 0..n {
			assert!(sccs.sccs.iter().any(|scc| *scc == vec![i]));
		}
	}
}
