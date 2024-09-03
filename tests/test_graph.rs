use library::graph::*;

#[test]
fn test_001_directed_graph() {
    // 辺を追加する -> 隣接辺を列挙 までが意図した通りに動くことを確認する
    let size = 500;
    let mut graph = DirectedAdjGraph::new(size);

    for i in 0..size {
        for j in i + 1..size {
            graph.add_edge(i, j, ());
        }
    }

    for i in 0..size {
        assert_eq!(graph.adjacent(i), &graph[i]);
        assert_eq!(
            graph.adjacent(i),
            &(i + 1..size).map(|v| (v, ())).collect::<Vec<_>>()
        );
    }

    let graph = graph.to_crs();

    for i in 0..size {
        assert_eq!(graph.adjacent(i), &graph[i]);
        assert_eq!(
            graph.adjacent(i),
            &(i + 1..size).map(|v| (v, ())).collect::<Vec<_>>()
        );
    }
}

#[test]
fn test_005_bfs() {
    // 辺の重みがないときは、bfsを行うことができる
    let graph = DirectedAdjGraph::from_edges_no_weight(5, &[(0, 1), (1, 2), (2, 3), (3, 4)]);

    assert_eq!(
        <dyn Graph::<Weight = ()>>::bfs(&graph, 0),
        vec![0, 1, 2, 3, 4]
    );
}
