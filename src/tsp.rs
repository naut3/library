use crate::unionfind::UnionFind;

/// 距離空間
pub trait MetricSpace {
    type P;
    type W: Ord + Copy;
    fn d(lhs: &Self::P, rhs: &Self::P) -> Self::W;
}

/// 2次元ユークリッド空間
pub struct EuclidianSpace2D {}
impl MetricSpace for EuclidianSpace2D {
    type P = (i32, i32);
    type W = u32;
    fn d(lhs: &Self::P, rhs: &Self::P) -> Self::W {
        let dx = i32::abs_diff(lhs.0, rhs.0);
        let dy = i32::abs_diff(lhs.1, rhs.1);
        let d = ((dx * dx + dy * dy) as f64).sqrt().round() as u32;
        return d;
    }
}

/// 3次元ユークリッド空間
pub struct EuclidianSpace3D {}
impl MetricSpace for EuclidianSpace3D {
    type P = (i32, i32, i32);
    type W = u32;
    fn d(lhs: &Self::P, rhs: &Self::P) -> Self::W {
        let dx = i32::abs_diff(lhs.0, rhs.0);
        let dy = i32::abs_diff(lhs.1, rhs.1);
        let dz = i32::abs_diff(lhs.2, rhs.2);
        let d = ((dx * dx + dy * dy + dz * dz) as f64).sqrt().round() as u32;
        return d;
    }
}

/// 距離空間における 2 近似の TSP 解法
pub fn tsp_two_approximation<S: MetricSpace>(points: &[S::P]) -> Vec<usize> {
    let size = points.len();
    let dist_matrix = (0..size)
        .map(|i| {
            (0..size)
                .map(|j| S::d(&points[i], &points[j]))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut uf = UnionFind::new(size);
    let mut hq = std::collections::BinaryHeap::new();
    let mut tree = vec![vec![]; size];
    for i in 0..size {
        for j in 0..size {
            hq.push((std::cmp::Reverse(dist_matrix[i][j]), i, j));
        }
    }
    while let Some((_, u, v)) = hq.pop() {
        if !uf.is_same(u, v) {
            uf.unite(u, v);
            tree[u].push(v);
            tree[v].push(u);
        }
    }
    let mut seen = vec![false; size];
    let mut path = vec![];
    let mut q = vec![];
    q.push(0);
    path.push(0);
    seen[0] = true;
    while let Some(u) = q.pop() {
        for &v in tree[u].iter() {
            if seen[v] {
                continue;
            }
            q.push(v);
            path.push(v);
            seen[v] = true;
        }
    }
    path.push(0);
    path
}
