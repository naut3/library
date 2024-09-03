use crate::graph::Graph;

pub fn cycle_detection(graph: &impl Graph) -> bool {
    struct DFS {
        seen: Vec<bool>,
        fin: Vec<bool>,
    }

    impl DFS {
        fn new(graph: &impl Graph) -> Self {
            assert!(graph.is_directed_edge());

            let size = graph.size();

            Self {
                seen: vec![false; size as usize],
                fin: vec![false; size as usize],
            }
        }

        fn run(&mut self, graph: &impl Graph) -> bool {
            for i in 0..graph.size() {
                if self.seen[i as usize] {
                    continue;
                }

                if self.dfs(graph, i) {
                    return true;
                }
            }

            false
        }

        fn dfs(&mut self, graph: &impl Graph, v: u32) -> bool {
            self.seen[v as usize] = true;

            for &(u, _) in graph.adjacent(v) {
                let u_us = u as usize;

                if self.fin[u_us] {
                    continue;
                }

                if self.seen[u_us] && !self.fin[u_us] {
                    return true;
                }

                if self.dfs(graph, u) {
                    return true;
                }
            }

            self.fin[v as usize] = true;
            false
        }
    }

    let mut dfs = DFS::new(graph);
    dfs.run(graph)
}
