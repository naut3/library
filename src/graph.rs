//! グラフを表現するトレイト・構造体を定義する。
//!
//! ## Usage
//! まず、隣接リストで表現されたグラフ [`AdjGraph`] を生成する。型エイリアスとして有向グラフは [`DirectedAdjGraph<W>`] が、無向グラフは [`UndirectedAdjGraph<W>`] があるので、それらを利用すると便利である。  
//! 具体的には、[`DirectedAdjGraph::new()`] や [`UndirectedAdjGraph::from_edges()`] などを利用すると良い。
//!
//! これらをそのまま使用することもできるが、キャッシュを考慮した別のメモリレイアウト [`CRSGraph`] に変換することもできる。[`AdjGraph::to_crs()`] でその変換を行える。  
//!
//! 実際の利用例としては、[Examples](#examples) の項を参考にするとよい。
//!
//! ## Examples
//!
//! 有向グラフを生成して、隣接辺を列挙する
//!
//! ```
//! use library::graph::DirectedAdjGraph;
//!
//! // 頂点数が 5 の有向グラフを隣接リスト形式で表す
//! let mut graph = DirectedAdjGraph::new(5);
//! // add_edge(u, v, w) の形で u --> v 間に重み w の辺を追加できる
//! graph.add_edge(0, 1, 1u32);
//! graph.add_edge(1, 2, 10);
//! graph.add_edge(3, 0, 100);
//! graph.add_edge(1, 4, 1000);
//!
//! // graph は有向グラフなので、3 --> 0 の辺は 0 の隣接辺ではない
//! assert_eq!(graph.adjacent(0), &vec![(1, 1)]);
//! // 添字によるアクセスもできる
//! assert_eq!(&graph[1], &vec![(2, 10), (4, 1000)]);
//! ```
//!
//! 重みなし無向グラフを生成して、最短距離を計算する
//!
//! ```
//! use library::graph::{Graph, UndirectedAdjGraph};
//!
//! let graph =
//!     UndirectedAdjGraph::from_edges_no_weight(6, &[(0, 1), (1, 2), (2, 0), (2, 4), (5, 4)]);
//! assert_eq!(&graph[2], vec![(1, ()), (0, ()), (4, ())]);
//!
//! // 同じグラフを保ったままメモリレイアウトを変更することもできる
//! let graph = graph.to_crs();
//! assert_eq!(&graph[4], vec![(2, ()), (5, ())]);
//!
//! let dist = <dyn Graph<Weight = ()>>::bfs(&graph, 0);
//! // 到達不可能な頂点の距離は `u32::MAX` になることに注意する
//! assert_eq!(dist, vec![0, 1, 1, u32::MAX, 2, 3]);
//! ```
//!
//! 木を生成して、最短距離を計算する
//!
//! * 与えられたグラフが木であることを確認せずに動作してしまうことに注意する。
//!
//! ```
//! use library::graph::{Tree, UndirectedAdjGraph};
//!
//! let graph = UndirectedAdjGraph::from_edges(
//!     6,
//!     &[
//!         (0, 1, 1),
//!         (2, 1, 10),
//!         (0, 3, 100),
//!         (4, 2, 1000),
//!         (2, 5, 10000),
//!     ],
//! );
//!
//! let dist = <dyn Tree<Weight = u32>>::dist(&graph, 0);
//! assert_eq!(dist, vec![0, 1, 11, 100, 1011, 10011]);
//! ```
//!

/// グラフの添字は `u32` で管理している
pub type Index = u32;

/// 有向グラフを隣接リスト形式で表現する構造体
pub type DirectedAdjGraph<W> = AdjGraph<Directed, W>;

/// 無向グラフを隣接リスト形式で表現する構造体
pub type UndirectedAdjGraph<W> = AdjGraph<Undirected, W>;

/// 有向グラフであることを示すトレイト
pub trait DirectedGraph: Graph {}

/// 無向グラフであることを示すトレイト
pub trait UndirectedGraph: Graph {}

/// (無向)木であることを示すトレイト
pub trait Tree: Graph {}

impl<W: Default + std::ops::Add<Output = W> + Copy> dyn Tree<Weight = W> {
    /// 木上で幅優先探索を行って、始点 `src` から他の頂点への最短距離を計算する。
    ///
    /// ## Example
    ///
    /// ```
    /// use library::graph::{Tree, UndirectedAdjGraph};
    ///
    /// let graph =
    ///     UndirectedAdjGraph::from_edges(5, &[(0, 1, 1), (1, 2, 10), (3, 0, 100), (0, 4, 1000)]);
    /// let dist = <dyn Tree<Weight = u32>>::dist(&graph, 1);
    ///
    /// assert_eq!(dist, vec![1, 0, 10, 101, 1001]);
    /// ```
    ///
    pub fn dist(&self, src: Index) -> Vec<W> {
        let size = self.size() as usize;
        let mut dist = vec![W::default(); size];
        let mut seen = vec![false; size];

        let mut q = std::collections::VecDeque::new();

        q.push_front(src);
        seen[src as usize] = true;

        while let Some(u) = q.pop_front() {
            let d = dist[u as usize];

            for &(v, w) in self.adjacent(u) {
                if seen[v as usize] {
                    continue;
                }

                q.push_front(v);
                seen[v as usize] = true;
                dist[v as usize] = d + w;
            }
        }

        dist
    }
}

pub trait Graph {
    /// 辺の重みの型を設定する
    type Weight;
    /// 自身の辺が有向辺かどうかを返す
    fn is_directed_edge(&self) -> bool;
    /// 自身の頂点数を返す
    fn size(&self) -> Index;
    /// `u` から `v` へ重み `w` の辺を新たに追加する
    fn add_edge(&mut self, u: Index, v: Index, w: Self::Weight);
    /// `v` から出ている辺を列挙する
    fn adjacent(&self, v: Index) -> &[(Index, Self::Weight)];
}

impl dyn Graph<Weight = ()> {
    pub fn bfs(&self, src: Index) -> Vec<Index> {
        let size = self.size();
        let mut seen = vec![false; size as usize];
        let mut dist = vec![Index::MAX; size as usize];
        let mut q = std::collections::VecDeque::new();

        dist[src as usize] = 0;
        seen[src as usize] = true;
        q.push_front(src);

        while let Some(u) = q.pop_front() {
            for &(nxt, _) in self.adjacent(u) {
                if seen[nxt as usize] {
                    continue;
                }

                dist[nxt as usize] = dist[u as usize] + 1;
                seen[nxt as usize] = true;
                q.push_back(nxt);
            }
        }

        dist
    }
}

/// 辺が有向か無向かを指し示す型マーカー
/// `Directed` か `Undirected` のいずれかである。
pub trait Orientation {
    fn is_directed_edge() -> bool;
}

pub enum Directed {}
impl Orientation for Directed {
    fn is_directed_edge() -> bool {
        true
    }
}
pub enum Undirected {}
impl Orientation for Undirected {
    fn is_directed_edge() -> bool {
        false
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct AdjGraph<O: Orientation, W> {
    size: Index,
    adj: Vec<Vec<(Index, W)>>,
    _marker: std::marker::PhantomData<O>,
}

impl<O: Orientation, W: Clone + Copy> AdjGraph<O, W> {
    /// construct a new graph, which has `size` vertices.
    pub fn new(size: Index) -> Self {
        Self {
            size,
            adj: vec![vec![]; size as usize],
            _marker: std::marker::PhantomData,
        }
    }

    /// add edge from `u` to `v` with weight `w`.
    pub fn add_edge(&mut self, u: Index, v: Index, w: W) -> () {
        if O::is_directed_edge() {
            self.adj[u as usize].push((v, w));
        } else {
            self.adj[u as usize].push((v, w.clone()));
            self.adj[v as usize].push((u, w));
        }
    }

    /// enumerate edges starting from vertex `v`.
    pub fn adjacent(&self, v: Index) -> &[(Index, W)] {
        &self[v]
    }

    pub fn from_edges(size: Index, edges: &[(Index, Index, W)]) -> Self {
        let mut graph = Self::new(size);

        for &(u, v, w) in edges {
            graph.add_edge(u, v, w);
        }

        graph
    }

    /// convert to CRSGraph
    pub fn to_crs(mut self) -> CRSGraph<O, W> {
        let mut crs = vec![];
        let mut ptr = vec![0];

        for i in 0..self.size as usize {
            crs.append(&mut self.adj[i]);
            ptr.push(crs.len() as Index);
        }

        CRSGraph {
            size: self.size,
            crs,
            ptr,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn size(&self) -> Index {
        self.size
    }

    pub fn is_directed_edge(&self) -> bool {
        O::is_directed_edge()
    }
}

impl<O: Orientation> AdjGraph<O, ()> {
    pub fn from_edges_no_weight(size: Index, edges: &[(Index, Index)]) -> Self {
        let mut graph = Self::new(size);

        for &(u, v) in edges {
            graph.add_edge(u, v, ())
        }

        graph
    }
}

impl<O: Orientation, W> std::ops::Index<Index> for AdjGraph<O, W> {
    type Output = [(Index, W)];
    fn index(&self, index: Index) -> &Self::Output {
        &self.adj[index as usize]
    }
}

impl<O: Orientation, W: Clone> Graph for AdjGraph<O, W> {
    type Weight = W;
    fn is_directed_edge(&self) -> bool {
        O::is_directed_edge()
    }
    fn add_edge(&mut self, u: Index, v: Index, w: Self::Weight) {
        if O::is_directed_edge() {
            self.adj[u as usize].push((v, w));
        } else {
            self.adj[u as usize].push((v, w.clone()));
            self.adj[v as usize].push((u, w));
        }
    }
    fn adjacent(&self, v: Index) -> &[(Index, Self::Weight)] {
        &self[v]
    }
    fn size(&self) -> Index {
        self.size
    }
}

impl<O: Orientation, W: Clone + Copy> From<AdjGraph<O, W>> for CRSGraph<O, W> {
    fn from(graph: AdjGraph<O, W>) -> Self {
        graph.to_crs()
    }
}

impl<W: Clone> DirectedGraph for AdjGraph<Directed, W> {}
impl<W: Clone> UndirectedGraph for AdjGraph<Undirected, W> {}
impl<W: Clone> Tree for AdjGraph<Undirected, W> {}

#[derive(Clone, PartialEq, Eq)]
pub struct CRSGraph<O: Orientation, W> {
    size: Index,
    crs: Vec<(Index, W)>,
    ptr: Vec<Index>,
    _marker: std::marker::PhantomData<O>,
}

impl<O: Orientation, W: Clone> CRSGraph<O, W> {
    pub fn new(_size: Index) -> Self {
        unreachable!()
    }

    pub fn add_edge(_u: Index, _v: Index, _w: W) -> () {
        unreachable!()
    }

    pub fn adjacent(&self, v: Index) -> &[(Index, W)] {
        &self[v]
    }

    pub fn size(&self) -> Index {
        self.size
    }

    pub fn is_directed_edge(&self) -> bool {
        O::is_directed_edge()
    }
}

impl<O: Orientation, W> std::ops::Index<Index> for CRSGraph<O, W> {
    type Output = [(Index, W)];
    fn index(&self, index: Index) -> &Self::Output {
        let v = index as usize;
        &self.crs[self.ptr[v] as usize..self.ptr[v + 1] as usize]
    }
}

impl<O: Orientation, W: Clone> Graph for CRSGraph<O, W> {
    type Weight = W;
    fn is_directed_edge(&self) -> bool {
        O::is_directed_edge()
    }
    fn add_edge(&mut self, _u: Index, _v: Index, _w: Self::Weight) {
        unreachable!()
    }
    fn adjacent(&self, v: Index) -> &[(Index, Self::Weight)] {
        &self[v]
    }
    fn size(&self) -> Index {
        self.size
    }
}

impl<W: Clone> DirectedGraph for CRSGraph<Directed, W> {}
impl<W: Clone> UndirectedGraph for CRSGraph<Undirected, W> {}
impl<W: Clone> Tree for CRSGraph<Undirected, W> {}
