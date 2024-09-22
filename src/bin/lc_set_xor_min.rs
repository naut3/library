// verification-helper: PROBLEM https://judge.yosupo.jp/problem/set_xor_min
#![allow(non_snake_case)]
use library::binary_trie::MultiBinaryTrie;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        Q: usize,
        query: [(u8, u64); Q],
    }

    let mut bt: MultiBinaryTrie<32> = MultiBinaryTrie::new();

    for (c, k) in query {
        match c {
            0 => {
                if bt.contains(k) {
                    continue;
                }
                bt.insert(k);
            }
            1 => {
                bt.remove(k);
            }
            2 => {
                let a = bt.xor_min(k).unwrap();
                println!("{}", a);
            }
            _ => unreachable!(),
        }
    }
}
