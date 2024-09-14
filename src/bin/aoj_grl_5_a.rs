// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/5/GRL_5_A
#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: u32,
        edges: [(u32, u32, u32); N - 1],
    }

    let graph = library::graph::UndirectedAdjGraph::from_edges(N, &edges);
    let diameter = library::tree_diameter::tree_diameter(&graph);
    println!("{}", diameter.dist);
}
