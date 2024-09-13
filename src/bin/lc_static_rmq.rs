// verification-helper: PROBLEM https://judge.yosupo.jp/problem/staticrmq
#![allow(non_snake_case)]
use library::algebra::Min;
use library::sparse_table::SparseTable;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize, Q: usize,
        A: [u32; N],
    }

    let st: SparseTable<Min<u32>> = SparseTable::from(&A);

    for _ in 0..Q {
        input! {
            l: usize, r: usize,
        }

        println!("{}", st.prod(l..r));
    }
}
