use library::binary_indexed_tree::BinaryIndexedTree;

#[test]
fn test_001_range() {
    let a = [1u32, 2, 3, 4, 5, 6, 7];

    let bit: BinaryIndexedTree<u32> = BinaryIndexedTree::from(&a);

    assert_eq!(bit.sum(0..=2), 6);
    assert_eq!(bit.sum(1..5), 14);
    assert_eq!(bit.sum(..4), 10);
    assert_eq!(bit.sum(3..), 22);
}

#[test]
fn test_002_lower_bound() {
    let a = [1, 10, 100, 1000, 10000];
    let bit = BinaryIndexedTree::from(&a);

    assert_eq!(bit.upper_bound(5), 1);
    assert_eq!(bit.upper_bound(0), 0);
    assert_eq!(bit.upper_bound(123), 3);
    assert_eq!(bit.upper_bound(11), 2);
}
