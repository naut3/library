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

/// 木の直径を求める。
///
/// * 木 $`T = (V, E)`$ の最遠頂点対の間のパスのことを木の直径という。
///
/// ## Usage
///
/// [`tree_diameter()`] を呼び出すと、[`Diameter`] 型の変数が返ってくる。これが直径に関する情報を持っているので、必要なものを利用する。
///
/// |関数・メンバ変数|内容|
/// |---|---|
/// |`path`| 直径をなすパス |
/// |`dist`| 直径をなすパスの重みの和 |
/// |`furthest_vertex_pair()`| 直径の両端の頂点 |
///
/// 具体的には、[Examples](#Examples) を見るとよい。
///
/// ## Examples
///
/// ```
/// use library::graph::UndirectedAdjGraph;
/// use library::tree_diameter::tree_diameter;
///
/// let graph = UndirectedAdjGraph::from_edges(
///     6,
///     &[
///         (0, 1, 1u32),
///         (1, 2, 100),
///         (1, 3, 10),
///         (0, 4, 1000),
///         (4, 5, 10000),
///     ],
/// );
/// let diameter = tree_diameter(&graph);
///
/// assert_eq!(diameter.path, [5, 4, 0, 1, 2]);
/// assert_eq!(diameter.dist, 11101);
/// assert_eq!(diameter.furthest_vertex_pair(), (5, 2));
/// ```
///
/// ## 計算量
///
/// 木 $`T = (V, E)`$ の辺の重みの型 `W` の加法が $`O(1)`$ で行えると仮定する。  
/// その上で、木の直径を計算する計算量は $`O(|V| + |E|)`$ である。
///
/// ## Verified problems
///
/// * [Tree Diameter](../../src/lc_tree_diameter/lc_tree_diameter.rs.html)
/// 
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
