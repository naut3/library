// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/6/NTL/1/NTL_1_A
#![allow(non_snake_case, unused_must_use, unused_imports)]
use library::prime_factorize::prime_factorize;
use std::io::{self, prelude::*};

fn main() {
    let (stdin, stdout) = (io::read_to_string(io::stdin()).unwrap(), io::stdout());
    let (mut stdin, mut buffer) = (stdin.split_whitespace(), io::BufWriter::new(stdout.lock()));

    macro_rules! input {
        ($t: tt, $n: expr) => {
            (0..$n).map(|_| input!($t)).collect::<Vec<_>>()
        };
        (Chars) => {
            input! {String}.chars().collect::<Vec<_>>()
        };
        (Usize1) => {
            stdin.next().unwrap().parse::<usize>().unwrap() - 1
        };
        ($t: ty) => {
            stdin.next().unwrap().parse::<$t>().unwrap()
        };
    }

    let n = input!(u64);

    let pf = prime_factorize(n);

    let mut ans = vec![];

    for (p, e) in pf {
        for _ in 0..e {
            ans.push(p);
        }
    }

    writeln!(
        buffer,
        "{}: {}",
        n,
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
