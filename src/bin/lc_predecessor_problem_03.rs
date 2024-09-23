// verification-helper: PROBLEM https://judge.yosupo.jp/problem/predecessor_problem
#![allow(non_snake_case)]
use library::fastset::FastSet;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        N: usize, Q: usize,
        T: Chars,
        query: [(u8, usize); Q],
    }

    let mut set = FastSet::new(N);

    for i in 0..N {
        if T[i] == '1' {
            set.insert(i);
        }
    }

    for (c, k) in query {
        match c {
            0 => {
                set.insert(k);
            }
            1 => {
                set.remove(k);
            }
            2 => {
                if set.contains(k) {
                    println!("1");
                } else {
                    println!("0");
                }
            }
            3 => {
                let nxt = set.next(k);

                if let Some(v) = nxt {
                    println!("{}", v);
                } else {
                    println!("-1");
                }
            }
            4 => {
                let prev = set.prev(k);

                if let Some(v) = prev {
                    println!("{}", v);
                } else {
                    println!("-1");
                }
            }
            _ => unreachable!(),
        }
    }
}
