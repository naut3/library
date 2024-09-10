// verification-helper: PROBLEM https://judge.yosupo.jp/problem/tree_diameter
#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize,
        edges: [(u32, u32, u64); N-1],
    }

    let tree = library::graph::UndirectedAdjGraph::from_edges(N as u32, &edges);

    let diameter = library::tree_diameter::tree_diameter(&tree);

    println!(
        "{} {}\n{}",
        diameter.dist,
        diameter.path.len(),
        diameter
            .path
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
