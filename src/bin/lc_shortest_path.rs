// verification-helper: PROBLEM https://judge.yosupo.jp/problem/shortest_path
#![allow(non_snake_case)]
use library::dijkstra::{dijkstras_algorithm_restore_path, Dist};
use library::graph::DirectedAdjGraph;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: u32, M: u32, s: u32, t: u32,
        edges: [(u32, u32, u64); M],
    }

    let graph = DirectedAdjGraph::from_edges(N, &edges);
    let (dist, path) = dijkstras_algorithm_restore_path(&graph, s, t);

    match dist {
        Dist::UNREACHABLE => {
            println!("-1");
        }
        Dist::VALUE(d) => {
            println!("{} {}", d, path.len() - 1);

            for i in 0..path.len() - 1 {
                println!("{} {}", path[i], path[i + 1]);
            }
        }
    }
}
