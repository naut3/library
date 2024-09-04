use library::graph::*;
use library::lowlink::LowLink;

#[test]
fn test_001_lowlink_construct() {
    let graph = UndirectedAdjGraph::from_edges_no_weight(4, &[(0, 1), (1, 2), (2, 0), (2, 3)]);

    let lowlink = LowLink::from(&graph);

    assert_eq!(lowlink.bridges(), &vec![(2, 3)]);
}
