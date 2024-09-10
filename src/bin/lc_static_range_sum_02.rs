// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_sum
#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize, Q: usize,
        A: [u64; N],
    }

    let bit = {
        let mut bit = library::binary_indexed_tree::BinaryIndexedTree::new(N);

        for i in 0..N {
            bit.add(i, A[i]);
        }

        bit
    };

    for _ in 0..Q {
        input! {
            l: usize, r: usize
        }

        println!("{}", bit.sum(l..r));
    }
}
