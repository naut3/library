use library::graph::{DirectedGraph, UndirectedGraph};

#[test]
fn test_001_directed_graph() {
    let mut graph = DirectedGraph::<()>::new(5);

    assert_eq!(graph.adjacent(0), &[]);

    graph.add_edge(0, 1, ());
    graph.add_edge(0, 3, ());
    assert_eq!(graph[0], [(1, ()), (3, ())]);

    graph.add_edge(2, 0, ());
    assert_eq!(graph[0], [(1, ()), (3, ())]);
}

#[test]
fn test_002_undirected_graph() {
    let mut graph = UndirectedGraph::<u32>::new(5);

    assert_eq!(graph[4], []);

    graph.add_edge(3, 4, 1);
    assert_eq!(graph[3], [(4, 1)]);
    assert_eq!(graph[4], [(3, 1)]);

    graph.add_edge(2, 3, 10);
    assert_eq!(graph[3], [(4, 1), (2, 10)]);
}

#[test]
fn test_003_convert_repr() {
    let size = 5;
    let mut graph = DirectedGraph::<()>::new(size);

    for i in 0..size {
        for j in i + 1..size {
            graph.add_edge(i, j, ());
        }
    }

    assert_eq!(graph[0], [(1, ()), (2, ()), (3, ()), (4, ())]);

    let graph = graph.to_crs();

    assert_eq!(graph[0], [(1, ()), (2, ()), (3, ()), (4, ())]);
    assert_eq!(graph[1], [(2, ()), (3, ()), (4, ())]);
}
