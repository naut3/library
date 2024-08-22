// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/4/GRL_4_A
#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: u32, M: usize,
        edges: [(u32, u32); M],
    }

    let mut graph = library::graph::DirectedGraph::<()>::new(N);

    for (u, v) in edges {
        graph.add_edge(u, v, ());
    }

    let has_cycle = library::cycle_detection::cycle_detection_directed(graph);

    println!("{}", if has_cycle { 1 } else { 0 });
}
