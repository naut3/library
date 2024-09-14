// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/5/GRL_5_B
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
    let (u, v) = diameter.furthest_vertex_pair();

    let dist_u = <dyn library::graph::Tree<Weight = u32>>::dist(&graph, u);
    let dist_v = <dyn library::graph::Tree<Weight = u32>>::dist(&graph, v);

    println!(
        "{}",
        (0..N as usize)
            .map(|i| std::cmp::max(dist_u[i], dist_v[i]))
            .collect::<Vec<_>>()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
}
