// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/3/GRL_3_B
#![allow(non_snake_case)]
use library::graph::*;
use library::lowlink::LowLink;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize, M: usize,
        edges: [(u32, u32); M],
    }

    let graph = UndirectedAdjGraph::from_edges_no_weight(N as u32, &edges);
    let lowlink = LowLink::from(&graph);

    let mut ans = lowlink.bridges().to_vec();
    ans.sort_unstable();

    // 空白とか改行コードとかの問題で、こう書かないとこの問題はACできない
    for (u, v) in ans {
        println!("{} {}", u, v);
    }
}
