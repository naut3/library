use library::binary_indexed_tree::BinaryIndexedTree;

#[test]
fn test_001_range() {
    let a = [1u32, 2, 3, 4, 5, 6, 7];

    let mut bit = BinaryIndexedTree::new(7);

    for i in 0..7 {
        bit.add(i, a[i]);
    }

    assert_eq!(bit.sum(0..=2), 6);
    assert_eq!(bit.sum(1..5), 14);
    assert_eq!(bit.sum(..4), 10);
    assert_eq!(bit.sum(3..), 22);
}
