// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/lesson/1/ALDS1/14/ALDS1_14_A
#![allow(non_snake_case)]
use library::rolling_hash::RollingHash;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        T: Chars,
        P: Chars,
    }

    let hash_t: RollingHash<'0', 100> = RollingHash::from(&T);
    let hash_p: RollingHash<'0', 100> = RollingHash::from(&P);
    let hp = hash_p.hash(..);

    let N = T.len();
    let M = P.len();

    let mut ans = vec![];

    for i in 0..N {
        if i + M <= N {
            if hash_t.hash(i..i + M) == hp {
                ans.push(i);
            }
        }
    }

    if ans.len() != 0 {
        println!(
            "{}",
            ans.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join("\n")
        );
    }
}
