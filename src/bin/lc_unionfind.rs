// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind
#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize, Q: usize,
    }

    let mut uf = library::unionfind::UnionFind::new(N);

    for _ in 0..Q {
        input! {
            t: u8, u: usize, v: usize,
        }

        if t == 0 {
            uf.unite(u, v);
        } else {
            println!("{}", if uf.is_same(u, v) { 1 } else { 0 });
        }
    }
}
