pub type Index = u32;

pub type DirectedAdjGraph<W> = AdjGraph<Directed, W>;
pub type UndirectedAdjGraph<W> = AdjGraph<Undirected, W>;

pub trait Graph {
    type Weight;
    fn is_directed_edge(&self) -> bool;
    fn size(&self) -> Index;
    fn add_edge(&mut self, u: Index, v: Index, w: Self::Weight);
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

pub trait DirectedGraph: Graph {}
pub trait UndirectedGraph: Graph {}

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
