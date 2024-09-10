// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/4/GRL_4_A
#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: u32, M: usize,
        edges: [(u32, u32); M],
    }

    let graph = library::graph::DirectedAdjGraph::from_edges_no_weight(N, &edges);
    let has_cycle = library::cycle_detection::cycle_detection(&graph);

    println!("{}", if has_cycle { 1 } else { 0 });
}
