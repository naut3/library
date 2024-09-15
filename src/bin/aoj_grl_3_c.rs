// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/3/GRL_3_C
#![allow(non_snake_case)]
use library::graph::DirectedAdjGraph;
use library::scc::strongly_connected_components;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: u32, M: u32,
        edges: [(u32, u32); M],
        Q: u32,
    }

    let graph = DirectedAdjGraph::from_edges_no_weight(N, &edges);
    let scc = strongly_connected_components(&graph);

    for _ in 0..Q {
        input! {
            u: usize, v: usize,
        }

        println!("{}", if scc[u] == scc[v] { 1 } else { 0 });
    }
}
