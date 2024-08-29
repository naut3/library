// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_kth_smallest
#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize, Q: usize,
        A: [u64; N],
    }

    let wm = library::wavelet_matrix::WaveletMatrix::from(&A, 30);

    for _ in 0..Q {
        input! {
            l: usize, r: usize, k: usize,
        }

        println!("{}", wm.quantile(l, r, k));
    }
}
