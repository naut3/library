use crate::graph::{Directed, Graph, Representation};

pub fn cycle_detection_directed<R: Representation<W = W>, W: Clone + Copy>(
    graph: Graph<Directed, R>,
) -> bool {
    struct DFS<R: Representation<W = W>, W: Clone + Copy> {
        graph: Graph<Directed, R>,
        seen: Vec<bool>,
        fin: Vec<bool>,
    }

    impl<R: Representation<W = W>, W: Clone + Copy> DFS<R, W> {
        fn from(graph: Graph<Directed, R>) -> bool {
            let size = graph.size;
            let mut g = Self {
                graph,
                seen: vec![false; size as usize],
                fin: vec![false; size as usize],
            };

            for i in 0..size {
                if g.seen[i as usize] {
                    continue;
                }

                let ret = g.dfs(i);

                if ret {
                    return true;
                }
            }

            false
        }

        fn dfs(&mut self, v: u32) -> bool {
            self.seen[v as usize] = true;

            for (nxt, _) in self.graph.adjacent(v).to_owned() {
                let nxt = nxt as usize;
                if self.fin[nxt] {
                    continue;
                }

                if self.seen[nxt] && !self.fin[nxt] {
                    return true;
                }

                if self.dfs(nxt as u32) {
                    return true;
                }
            }

            self.fin[v as usize] = true;

            return false;
        }
    }

    DFS::from(graph)
}
