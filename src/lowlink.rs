use crate::graph::UndirectedGraph;

/// [`LowLink`] は、連結グラフ $`G = (V, E)`$ の関節点や橋を $`O(|V| + |E|)`$ で検出することができる。  
///
/// * 連結グラフ $`G = (V, E)`$ で頂点 $`v \in V`$ とそれから伸びている辺を取り除くと、グラフが非連結になるとき、その頂点 $`v`$ は関節点であるという。
/// * 連結グラフ $`G = (V, E)`$ で辺 $`e \in E`$ を取り除くと、グラフが非連結になるとき、その辺 $`e`$ は橋であるという。
///
/// ## Examples
///
/// ```
/// use library::graph::UndirectedAdjGraph;
/// use library::lowlink::LowLink;
///
/// let graph =
///     UndirectedAdjGraph::from_edges_no_weight(5, &[(0, 1), (1, 2), (2, 0), (1, 3), (4, 3)]);
/// let lowlink = LowLink::from(&graph);
///
/// assert_eq!(lowlink.articulation_points(), [3, 1]);
/// assert_eq!(lowlink.bridges(), [(3, 4), (1, 3)]);
/// ```
///
/// ## Verified problems
///
/// * [Articulation Points](../../src/aoj_grl_3_a/aoj_grl_3_a.rs.html)
/// * [Bridges](../../src/aoj_grl_3_b/aoj_grl_3_b.rs.html)
///

pub struct LowLink {
    seen: Vec<bool>,
    ord: Vec<u32>,
    low: Vec<u32>,
    articulation_points: Vec<u32>,
    bridges: Vec<(u32, u32)>,
}

impl LowLink {
    const ROOT: u32 = 1 << 30;

    /// `graph` を受け取って、関節点、橋を求める。
    pub fn from(graph: &impl UndirectedGraph) -> Self {
        let size = graph.size();
        let mut lowlink = Self {
            seen: vec![false; size as usize],
            ord: vec![size; size as usize],
            low: vec![size; size as usize],
            articulation_points: vec![],
            bridges: vec![],
        };

        for i in 0..size {
            lowlink.dfs(graph, i, Self::ROOT, 0);
        }

        lowlink
    }

    fn dfs(&mut self, graph: &impl UndirectedGraph, v: u32, parent: u32, mut cnt: u32) {
        if self.seen[v as usize] {
            return;
        }

        self.seen[v as usize] = true;
        self.ord[v as usize] = cnt;
        self.low[v as usize] = cnt;
        cnt += 1;

        let mut child_cnt = 0;
        let mut is_articulation_point = false;

        for &(u, _) in graph.adjacent(v) {
            if self.seen[u as usize] {
                if u != parent {
                    self.low[v as usize] =
                        std::cmp::min(self.low[v as usize], self.ord[u as usize]);
                }
            } else {
                child_cnt += 1;
                self.dfs(graph, u, v, cnt);

                if u != parent {
                    self.low[v as usize] =
                        std::cmp::min(self.low[v as usize], self.low[u as usize]);
                }

                if parent != Self::ROOT && self.ord[v as usize] <= self.low[u as usize] {
                    is_articulation_point = true;
                }

                if self.ord[v as usize] < self.low[u as usize] {
                    let (a, b) = (std::cmp::min(u, v), std::cmp::max(u, v));
                    self.bridges.push((a, b));
                }
            }
        }

        if parent == Self::ROOT && child_cnt >= 2 {
            is_articulation_point = true;
        }

        if is_articulation_point {
            self.articulation_points.push(v);
        }
    }

    /// 求めた関節点を列挙する。
    pub fn articulation_points(&self) -> &[u32] {
        &self.articulation_points
    }

    /// 求めた橋を列挙する。
    pub fn bridges(&self) -> &[(u32, u32)] {
        &self.bridges
    }
}
