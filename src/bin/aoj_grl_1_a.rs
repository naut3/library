// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/1/GRL_1_A
#![allow(non_snake_case)]
use library::dijkstra::{dijkstras_algorithm, Dist};
use library::graph::DirectedAdjGraph;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: u32, M: u32, src: u32,
        edges: [(u32, u32, u32); M],
    }

    let graph = DirectedAdjGraph::from_edges(N, &edges);
    let res = dijkstras_algorithm(&graph, src);

    for i in 0..N {
        match res.get(i) {
            Dist::UNREACHABLE => {
                println!("INF");
            }
            Dist::VALUE(d) => {
                println!("{}", d);
            }
        }
    }
}
