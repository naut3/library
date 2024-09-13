use library::algebra::Min;
use library::sparse_table::SparseTable;

#[test]
fn test_000_elem_nothing() {
    let a = [];
    let _: SparseTable<Min<u32>> = SparseTable::from(&a);
}

#[test]
fn test_001() {
    let a = [7, 6, 5, 4, 3, 2, 1];
    let st: SparseTable<Min<u32>> = SparseTable::from(&a);

    assert_eq!(st.prod(0..=1), 6);
    assert_eq!(st.prod(2..5), 3);
    assert_eq!(st.prod(1..), 1);
}
