// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/lesson/8/ITP2/6/ITP2_6_A
#![allow(non_snake_case)]
use library::binary_trie::MultiBinaryTrie;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize,
        A: [u64; N],
        Q: usize,
    }

    let mut bt: MultiBinaryTrie<30> = MultiBinaryTrie::new();

    for a in A {
        bt.insert(a);
    }

    for _ in 0..Q {
        input! {
            k: u64,
        }

        println!("{}", if bt.contains(k) { 1 } else { 0 });
    }
}
