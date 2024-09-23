// verification-helper: PROBLEM https://judge.yosupo.jp/problem/predecessor_problem
#![allow(non_snake_case)]
use library::binary_indexed_tree::BinaryIndexedTree;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        N: usize, Q: usize,
        T: Chars,
        query: [(u8, usize); Q],
    }

    let mut bit = BinaryIndexedTree::from(
        &(0..N)
            .map(|i| if T[i] == '1' { 1i32 } else { 0 })
            .collect::<Vec<_>>(),
    );

    for (c, k) in query {
        match c {
            0 => {
                if bit.sum(k..=k) == 0 {
                    bit.add(k, 1);
                }
            }
            1 => {
                if bit.sum(k..=k) == 1 {
                    bit.add(k, -1);
                }
            }
            2 => {
                println!("{}", if bit.sum(k..=k) == 1 { 1 } else { 0 });
            }
            3 => {
                // k 未満の数の集合が空になっていることに注意
                let s = if k != 0 { bit.sum(0..k) } else { 0 };
                let ans = bit.upper_bound(s);
                if ans == N {
                    println!("-1");
                } else {
                    println!("{}", ans);
                }
            }
            4 => {
                let s = bit.sum(0..=k);

                if s == 0 {
                    println!("-1");
                } else {
                    let ans = bit.upper_bound(s - 1);
                    println!("{}", ans);
                }
            }
            _ => unreachable!(),
        }
    }
}
