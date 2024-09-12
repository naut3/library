//! 非負の重みをもつ辺のグラフに対する最短経路問題を解くことができる
//!
//! ## Examples
//!
//! 与えられたグラフ $`G = (V, E)`$ で始点から他の頂点への最短距離を計算する。  
//! 計算した結果は、[`DijkstraResult`] 型の変数に格納される。頂点 $`i \in V`$ の結果は `dijkstraresult.get(i)` で取得できる。
//!  
//! 返り値は [`Dist`] 型である。   
//! 到達できず、最短距離が存在しない場合がある。そのとき、距離は `Dist::UNREACHABLE` になる。  
//! また、到達可能で最短距離が `d` の場合は、`Dist::VALUE(d)` になる。
//!
//! ```
//! use library::dijkstra::{dijkstras_algorithm, Dist};
//! use library::graph::DirectedAdjGraph;
//!
//! let graph = DirectedAdjGraph::from_edges(
//!     5,
//!     &[(0, 1, 1u16), (1, 2, 10), (2, 4, 100), (3, 0, 1), (3, 1, 2)],
//! );
//!
//! let res = dijkstras_algorithm(&graph, 0);
//!
//! assert_eq!(res.get(0), Dist::VALUE(0));
//! assert_eq!(res.get(1), Dist::VALUE(1));
//! assert_eq!(res.get(2), Dist::VALUE(11));
//! assert_eq!(res.get(3), Dist::UNREACHABLE);
//! assert_eq!(res.get(4), Dist::VALUE(111));
//! ```
//!
//! グラフ $`G = (V, E)`$ で始点 $`s \in V`$ から終点 $`t \in V`$ への最短経路のうちの一つを構成することもできる。
//!
//! 2つ目の返り値は、$`p_0, p_1, \dots, p_{L-1}`$ のようになっているとして、$`p_0 = s, p_{L - 1} = t`$ かつ $`p_i`$ から $`p_{i + 1}`$ への辺が必ず存在するパスで、これは最短経路の内の一つである。
//!
//! ```
//! use library::dijkstra::{dijkstras_algorithm_restore_path, Dist};
//! use library::graph::DirectedAdjGraph;
//!
//! let graph = DirectedAdjGraph::from_edges(
//!     5,
//!     &[
//!         (0, 1, 1u16),
//!         (1, 2, 10),
//!         (1, 3, 100),
//!         (3, 4, 1000),
//!         (2, 4, 10000),
//!     ],
//! );
//!
//! let (dist, path) = dijkstras_algorithm_restore_path(&graph, 0, 4);
//!
//! assert_eq!(dist, Dist::VALUE(1101));
//! assert_eq!(path, vec![0, 1, 3, 4]);
//! ```
//!
//! ## 計算量
//!
//! `W` の空間計算量が $`O(1)`$ で、加法が $`O(1)`$ で行えることを仮定する。
//!
//! [`dijkstras_algorithm`], [`dijkstras_algorithm_restore_path`] いずれも引数の `graph` が $`G = (V, E)`$ であるとして、$`O((|V| + |E|) \log{|V|})`$ である。
//!
//! ## Verified problems
//!
//! * [Single Source Shortest Path](../../src/aoj_grl_1_a/aoj_grl_1_a.rs.html)
//! * [Shortest Path](../../src/lc_shortest_path/lc_shortest_path.rs.html)
//!

use crate::graph::Graph;
use crate::integer_traits::HasMaxValue;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Dist<W> {
    /// 到達不可能な場合
    UNREACHABLE,
    /// 到達可能な場合 -> 最短距離
    VALUE(W),
}

/// [`dijkstras_algorithm`] の結果を格納するデータ構造
pub struct DijkstraResult<W> {
    seen: Vec<bool>,
    dist: Vec<W>,
}

impl<W: Copy> DijkstraResult<W> {
    pub fn get(&self, i: u32) -> Dist<W> {
        if self.seen[i as usize] {
            Dist::VALUE(self.dist[i as usize])
        } else {
            Dist::UNREACHABLE
        }
    }
}

/// `graph` 上で始点 `src` から各頂点への最短距離を計算する
pub fn dijkstras_algorithm<W: Default + std::ops::Add<Output = W> + Ord + Copy + HasMaxValue>(
    graph: &impl Graph<Weight = W>,
    src: u32,
) -> DijkstraResult<W> {
    let size = graph.size();

    let mut hq = std::collections::BinaryHeap::new();
    let mut seen = vec![false; size as usize];
    let mut dist = vec![W::MAX; size as usize];
    let mut seen_cnt = 0;

    hq.push((std::cmp::Reverse(W::default()), src));
    dist[src as usize] = W::default();

    while let Some((_, u)) = hq.pop() {
        if seen[u as usize] {
            continue;
        }
        seen[u as usize] = true;
        seen_cnt += 1;

        if seen_cnt == size {
            break;
        }

        for &(v, w) in graph.adjacent(u) {
            if !seen[v as usize] {
                let dv = dist[u as usize] + w;

                if dv < dist[v as usize] {
                    dist[v as usize] = dv;
                    hq.push((std::cmp::Reverse(dv), v));
                }
            }
        }
    }

    DijkstraResult { seen, dist }
}

/// `graph` 上で始点 `src` から終点 `dst` への最短経路を計算する
pub fn dijkstras_algorithm_restore_path<
    W: Default + std::ops::Add<Output = W> + Ord + Copy + HasMaxValue,
>(
    graph: &impl Graph<Weight = W>,
    src: u32,
    dst: u32,
) -> (Dist<W>, Vec<u32>) {
    let size = graph.size();

    let mut hq = std::collections::BinaryHeap::new();
    let mut seen = vec![false; size as usize];
    let mut dist = vec![W::MAX; size as usize];
    let mut prev = vec![u32::MAX; size as usize];

    hq.push((std::cmp::Reverse(W::default()), src));
    dist[src as usize] = W::default();

    while let Some((_, u)) = hq.pop() {
        if seen[u as usize] {
            continue;
        }
        seen[u as usize] = true;

        if u == dst {
            break;
        }

        for &(v, w) in graph.adjacent(u) {
            if !seen[v as usize] {
                let dv = dist[u as usize] + w;

                if dv < dist[v as usize] {
                    dist[v as usize] = dv;
                    hq.push((std::cmp::Reverse(dv), v));
                    prev[v as usize] = u;
                }
            }
        }
    }

    if !seen[dst as usize] {
        return (Dist::UNREACHABLE, vec![]);
    }

    let mut path = vec![dst];
    let mut v = dst;

    while v != src {
        v = prev[v as usize];
        path.push(v);
    }
    path.reverse();

    return (Dist::VALUE(dist[dst as usize]), path);
}
