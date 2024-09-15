// verification-helper: PROBLEM https://judge.yosupo.jp/problem/scc
#![allow(non_snake_case)]
use library::graph::DirectedAdjGraph;
use library::scc::strongly_connected_components;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: u32, M: u32,
        edges: [(u32, u32); M],
    }

    let graph = DirectedAdjGraph::from_edges_no_weight(N, &edges);
    let scc = strongly_connected_components(&graph);

    let mut components = vec![vec![]; *scc.iter().max().unwrap() as usize + 1];

    for i in 0..N as usize {
        components[scc[i] as usize].push(i);
    }

    println!("{}", components.len());

    for c in components {
        println!(
            "{} {}",
            c.len(),
            c.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}
