// verification-helper: PROBLEM https://judge.yosupo.jp/problem/predecessor_problem
#![allow(non_snake_case)]
use library::binary_trie::MultiBinaryTrie;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        N: usize, Q: usize,
        T: Chars,
        query: [(u8, u64); Q],
    }

    let mut bt: MultiBinaryTrie<30> = MultiBinaryTrie::new();

    for i in 0..N {
        if T[i] == '1' {
            bt.insert(i as u64);
        }
    }

    for (c, k) in query {
        match c {
            0 => {
                if !bt.contains(k) {
                    bt.insert(k)
                }
            }
            1 => {
                bt.remove(k);
            }
            2 => {
                println!("{}", if bt.contains(k) { 1 } else { 0 });
            }
            3 => {
                let v = bt.lower_bound(k);

                if let Some(v) = v {
                    println!("{}", v);
                } else {
                    println!("-1");
                }
            }
            4 => {
                let v = bt.upper_bound(k);

                if let Some(v) = v {
                    println!("{}", v);
                } else {
                    println!("-1");
                }
            }
            _ => unreachable!(),
        }
    }
}
