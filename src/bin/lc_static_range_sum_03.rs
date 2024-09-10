// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_sum
#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize, Q: usize,
        A: [u64; N],
    }

    let wm = library::wavelet_matrix::WaveletMatrix::from_weighted_own(&A, 30);

    for _ in 0..Q {
        input! {
            l: usize, r: usize
        }

        println!("{}", wm.sum(l, r));
    }
}
