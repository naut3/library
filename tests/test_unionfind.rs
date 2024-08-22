use library::unionfind::UnionFind;

#[test]
fn test_001_size_check() {
    let mut uf = UnionFind::new(5);

    assert_eq!(uf.size(0), 1);

    uf.unite(0, 1);
    assert_eq!(uf.size(0), 2);

    uf.unite(2, 3);
    uf.unite(3, 4);
    uf.unite(0, 4);
    assert_eq!(uf.size(0), 5);
}
