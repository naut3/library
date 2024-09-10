// verification-helper: PROBLEM https://judge.yosupo.jp/problem/lca
#![allow(non_snake_case)]
use library::doubling::Doubling;
use library::graph::{Graph, UndirectedAdjGraph};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize, Q: usize,
        mut P: [u32; N - 1],
        query: [(u32, u32); Q],
    }

    let mut graph = UndirectedAdjGraph::new(N as u32);

    for i in 1..=N - 1 {
        graph.add_edge(i as u32, P[i - 1], ());
    }

    let dist = <dyn Graph<Weight = ()>>::bfs(&graph, 0);

    let mut A = vec![0];
    A.append(&mut P);

    let dbl = Doubling::build(&A, 20);

    for (mut u, mut v) in query {
        let du = dist[u as usize];
        let dv = dist[v as usize];
        let dm = std::cmp::min(du, dv);

        u = dbl.next(u, (du - dm) as u64);
        v = dbl.next(v, (dv - dm) as u64);

        if u == v {
            println!("{}", u);
            continue;
        }

        for i in (0..20).rev() {
            let (pu, pv) = (dbl.jump_power_of_two(u, i), dbl.jump_power_of_two(v, i));

            if pu != pv {
                u = pu;
                v = pv;
            }
        }

        println!("{}", dbl.next(u, 1));
    }
}
