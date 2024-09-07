use crate::graph::Tree;

pub struct Diameter<W> {
    pub dist: W,
    src: u32,
    dst: u32,
    pub path: Vec<u32>,
}

impl<W> Diameter<W> {
    pub fn furthest_vertex_pair(&self) -> (u32, u32) {
        (self.src, self.dst)
    }
}

pub fn tree_diameter<W: Default + Copy + Ord + std::ops::Add<Output = W>>(
    tree: &dyn Tree<Weight = W>,
) -> Diameter<W> {
    let size = tree.size() as usize;

    let mut seen: Vec<u8> = vec![0; size];
    let mut dist = vec![W::default(); size];
    let mut q = std::collections::VecDeque::new();

    let r1 = {
        let flag = 1;
        seen[0] = flag;
        dist[0] = W::default();
        q.push_front(0);

        while let Some(u) = q.pop_front() {
            for &(v, w) in tree.adjacent(u) {
                if seen[v as usize] == flag {
                    continue;
                }

                seen[v as usize] = flag;
                dist[v as usize] = dist[u as usize] + w;
                q.push_front(v);
            }
        }

        (0..size).max_by_key(|&i| dist[i]).unwrap() as u32
    };

    let flag = 2;
    seen[r1 as usize] = flag;
    dist[r1 as usize] = W::default();
    let mut prev = vec![u32::MAX; size];

    q.push_front(r1);

    while let Some(u) = q.pop_front() {
        for &(v, w) in tree.adjacent(u) {
            if seen[v as usize] == flag {
                continue;
            }

            seen[v as usize] = flag;
            dist[v as usize] = dist[u as usize] + w;
            q.push_front(v);
            prev[v as usize] = u;
        }
    }

    let r2 = (0..size).max_by_key(|&i| dist[i]).unwrap() as u32;

    let dist = dist[r2 as usize];

    let mut path = vec![r2];
    let mut v = r2;

    while v != r1 {
        v = prev[v as usize];
        path.push(v);
    }

    path.reverse();

    Diameter {
        dist,
        src: r1,
        dst: r2,
        path,
    }
}
