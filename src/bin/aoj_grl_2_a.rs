// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/2/GRL_2_A
#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize, M: usize,
        mut edges: [(usize, usize, u32); M],
    }

    let mut uf = library::unionfind::UnionFind::new(N);
    edges.sort_by_key(|&(_, _, w)| w);

    let ans = edges
        .iter()
        .map(|&(u, v, w)| {
            if uf.is_same(u, v) {
                0
            } else {
                uf.unite(u, v);
                w
            }
        })
        .sum::<u32>();

    println!("{}", ans);
}
