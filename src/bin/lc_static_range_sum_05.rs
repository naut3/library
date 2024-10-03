// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_sum
#![allow(non_snake_case)]
use library::algebra::Add;
use library::segtree::SegmentTree;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize, Q: usize,
        A: [u64; N],
    }

    let stree: SegmentTree<Add<u64>> = SegmentTree::from(&A);

    for _ in 0..Q {
        input! {
            l: usize, r: usize,
        }

        println!("{}", stree.prod(l..r));
    }
}
