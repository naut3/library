use crate::graph::{DirectedAdjGraph, DirectedGraph};

/// 強連結成分分解をする  
///
/// 有向グラフ `graph` を強連結成分分解する。強連結成分に分解したグラフは有向非巡回グラフであるから、トポロジカルソートができる。  
/// トポロジカル順序で強連結成分を順に $`0, 1, 2, \dots`$ と番号付けし、各頂点が何番目の強連結成分に含まれるかを計算する。
///
/// ## Examples
///
/// ```
/// use library::graph::DirectedAdjGraph;
/// use library::scc::strongly_connected_components;
///
/// // graph は強連結成分として {0, 1} -> {2, 3, 4} -> {5} のようになっている
/// let graph = DirectedAdjGraph::from_edges_no_weight(
///     6,
///     &[(0, 1), (1, 0), (1, 2), (2, 3), (3, 4), (4, 2), (4, 5)],
/// );
/// let scc = strongly_connected_components(&graph);
///
/// // 同じ成分に属する組
/// assert_eq!(scc[0], scc[1]);
/// assert_eq!(scc[2], scc[3]);
/// assert_eq!(scc[3], scc[4]);
///
/// // 異なる成分に属する組
/// assert_ne!(scc[5], scc[0]);
/// assert_ne!(scc[5], scc[2]);
/// assert_ne!(scc[0], scc[2]);
///
/// // 成分同士の比較
/// assert!(scc[0] < scc[2]);
/// assert!(scc[2] < scc[5]);
/// ```
///
/// ## 計算量
///
/// 有向グラフ `graph` が $`G = (V, E)`$ であるとする。このとき、$`O(|V| + |E|)`$ である。
///
/// ## Verified problems
///
/// * [Strongly Connected Components(Aizu Online Judge)](../../src/aoj_grl_3_c/aoj_grl_3_c.rs.html)
/// * [Strongly Connected Components(Library Checker)](../../src/lc_scc/lc_scc.rs.html)
///
pub fn strongly_connected_components<T>(graph: &dyn DirectedGraph<Weight = T>) -> Vec<u32> {
    struct DFS {
        seen: Vec<bool>,
        stop: Vec<u32>,
    }

    impl DFS {
        fn new<T>(graph: &dyn DirectedGraph<Weight = T>) -> Self {
            Self {
                seen: vec![false; graph.size() as usize],
                stop: vec![],
            }
        }

        fn dfs<T>(&mut self, v: u32, graph: &dyn DirectedGraph<Weight = T>) {
            self.seen[v as usize] = true;

            for &(u, _) in graph.adjacent(v) {
                if self.seen[u as usize] {
                    continue;
                }

                self.dfs(u, graph);
            }

            self.stop.push(v);
        }
    }

    let mut dfs = DFS::new(graph);

    for i in 0..graph.size() {
        if !dfs.seen[i as usize] {
            dfs.dfs(i, graph);
        }
    }

    let mut graph_inv = DirectedAdjGraph::new(graph.size());

    for i in 0..graph.size() {
        for &(u, _) in graph.adjacent(i) {
            graph_inv.add_edge(u, i, ());
        }
    }

    let mut dfs_inv = DFS::new(&graph_inv);

    let mut stop = dfs.stop;
    stop.reverse();

    let mut id = vec![0; graph.size() as usize];

    let mut cnt = 0;

    for u in stop {
        if dfs_inv.seen[u as usize] {
            continue;
        }

        dfs_inv.dfs(u, &graph_inv);

        for &v in dfs_inv.stop.iter() {
            id[v as usize] = cnt;
        }

        cnt += 1;
        dfs_inv.stop.clear();
    }

    id
}
