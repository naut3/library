use library::cumulative_sum::CumulativeSum;

#[test]
fn test_001_range() {
    let a: [u32; 5] = [1, 10, 100, 1000, 10000];
    let cs = CumulativeSum::from(&a);

    assert_eq!(cs.sum(0..1), 1);
    assert_eq!(cs.sum(1..3), 110);

    assert_eq!(cs.sum(3..=4), 11000);

    assert_eq!(cs.sum(2..), 11100);
    assert_eq!(cs.sum(..3), 111);
    assert_eq!(cs.sum(..), 11111);
}
