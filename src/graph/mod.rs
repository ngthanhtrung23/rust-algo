pub mod scc;

// Common graph representation.
struct Graph {
	n_vertex: usize,
	adjs: Vec<Vec<usize>>,
}

impl Graph {
	pub fn new(v: usize) -> Self {
		Self {
			n_vertex: v,
			adjs: vec![vec![]; v]
		}
	}

	pub fn add_directed_edge(&mut self, u: usize, v:usize) {
		self.adjs[u].push(v);
	}
}

#[cfg(test)]
mod test {
	use graph::Graph;

	#[test]
	fn add_directed_edge() {
		let mut graph = Graph::new(2);
		graph.add_directed_edge(0, 1);

		assert_eq!(graph.adjs[0].len(), 1);
		assert_eq!(graph.adjs[1].len(), 0);
	}
}
