pub type DirectedGraph<W> = Graph<Directed, AdjacencyList<W>>;
pub type UndirectedGraph<W> = Graph<Undirected, AdjacencyList<W>>;

type Index = u32;

#[derive(Clone, PartialEq, Eq)]
pub struct Graph<O: Orientation, R: Representation> {
    pub size: Index,
    representation: R,
    _marker: std::marker::PhantomData<O>,
}

impl<O: Orientation, R: Representation<W = W>, W: Clone + Copy> Graph<O, R> {
    /// make a new graph with `size` vertices
    pub fn new(size: Index) -> Self {
        Self {
            size,
            representation: R::new(size),
            _marker: std::marker::PhantomData,
        }
    }

    /// add an edge of weight `w` between `u` and `v`
    pub fn add_edge(&mut self, u: Index, v: Index, w: W) {
        self.representation.add_edge(u, v, w.clone());

        if O::is_directed() {
            return;
        } else {
            self.representation.add_edge(v, u, w);
        }
    }

    /// enumerate vertices adjacent to vertex `v`
    pub fn adjacent(&self, v: Index) -> &[(Index, W)] {
        self.representation.adjacent(v)
    }

    pub fn from_edges(size: Index, edges: &[(Index, Index, W)]) -> Self {
        let mut graph = Self::new(size);

        for &(u, v, w) in edges {
            graph.add_edge(u, v, w);
        }

        graph
    }
}

impl<O: Orientation, R: Representation<W = W>, W: Clone + Copy> std::ops::Index<Index>
    for Graph<O, R>
{
    type Output = [(Index, W)];

    fn index(&self, index: Index) -> &Self::Output {
        self.adjacent(index)
    }
}

impl<O: Orientation, W: Clone> Graph<O, AdjacencyList<W>> {
    pub fn to_crs(self) -> Graph<O, CrsList<W>> {
        Graph {
            size: self.size,
            representation: CrsList::from(self.representation),
            _marker: std::marker::PhantomData,
        }
    }
}

pub trait Representation {
    type W;
    fn new(size: Index) -> Self;
    fn adjacent(&self, v: Index) -> &[(Index, Self::W)];
    fn add_edge(&mut self, u: Index, v: Index, w: Self::W);
}

#[derive(Clone, PartialEq, Eq)]
pub struct AdjacencyList<W> {
    list: Vec<Vec<(Index, W)>>,
}

impl<W: Clone> Representation for AdjacencyList<W> {
    type W = W;
    fn new(size: Index) -> Self {
        Self {
            list: vec![vec![]; size as usize],
        }
    }
    fn adjacent(&self, v: Index) -> &[(Index, Self::W)] {
        &self.list[v as usize]
    }
    fn add_edge(&mut self, u: Index, v: Index, w: Self::W) {
        self.list[u as usize].push((v, w));
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct CrsList<W> {
    list: Vec<(Index, W)>,
    ptr: Vec<Index>,
}

impl<W> From<AdjacencyList<W>> for CrsList<W> {
    fn from(mut value: AdjacencyList<W>) -> Self {
        let mut list = vec![];
        let mut ptr = vec![0];

        let size = value.list.len();

        for i in 0..size {
            list.append(&mut value.list[i]);
            ptr.push(list.len() as Index);
        }

        Self { list, ptr }
    }
}

impl<W> Representation for CrsList<W> {
    type W = W;
    fn new(_size: Index) -> Self {
        unreachable!();
    }
    fn adjacent(&self, v: Index) -> &[(Index, Self::W)] {
        &self.list[self.ptr[v as usize] as usize..self.ptr[v as usize + 1] as usize]
    }
    fn add_edge(&mut self, _u: Index, _v: Index, _w: Self::W) {
        unreachable!();
    }
}

/// type marker of edges orientation
pub trait Orientation {
    fn is_directed() -> bool;
}

pub enum Directed {}
impl Orientation for Directed {
    fn is_directed() -> bool {
        true
    }
}

pub enum Undirected {}
impl Orientation for Undirected {
    fn is_directed() -> bool {
        false
    }
}
