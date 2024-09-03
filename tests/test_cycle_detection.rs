use library::cycle_detection::*;
use library::graph::*;

#[test]
fn test_001_cycle_detection_directed() {
    let graph = DirectedAdjGraph::from_edges_no_weight(5, &[(0, 1), (1, 2), (2, 3), (3, 4)]);
    let has_cycle = cycle_detection(&graph);
    assert_eq!(has_cycle, false);

    let graph =
        DirectedAdjGraph::from_edges_no_weight(5, &[(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)]);
    let has_cycle = cycle_detection(&graph);
    assert_eq!(has_cycle, true);
}
