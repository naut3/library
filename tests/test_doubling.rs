use library::doubling::Doubling;

#[test]
fn test_001_one_elem() {
    let nxt = vec![0];

    // depth = 30 なら、2^30 回遷移した先までを事前計算する。したがって、2^31 より小さい遷移回数なら、遷移先を計算できる。
    let dbl = Doubling::build(&nxt, 30);
    assert_eq!(dbl.next(0, (1 << 31) - 1), 0);
}

#[test]
fn test_002_mini() {
    let nxt = vec![1, 2, 0];

    let dbl = Doubling::build(&nxt, 30);
    assert_eq!(dbl.next(0, 1), 1);
    assert_eq!(dbl.next(0, 2), 2);
    assert_eq!(dbl.next(0, 3 << 15), 0);
}
