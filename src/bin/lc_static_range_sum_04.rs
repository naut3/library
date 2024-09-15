// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_sum
#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize, Q: usize,
        A: [u64; N],
    }

    let mut dbit = library::dynamic_binary_indexed_tree::DynamicBinaryIndexedTree::new(N);

    for i in 0..N {
        dbit.add(i, A[i]);
    }

    for _ in 0..Q {
        input! {
            l: usize, r: usize
        }

        println!("{}", dbit.sum(l..r));
    }
}
