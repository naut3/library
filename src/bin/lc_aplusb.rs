// verification-helper: PROBLEM https://judge.yosupo.jp/problem/aplusb
#![allow(non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        A: u32, B: u32,
    }

    println!("{}", A + B);
}
