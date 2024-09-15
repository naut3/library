/// [`BinaryIndexedTree`]は一点の値の更新と区間和の計算を高速に行うことができる。
///  
/// ## Examples
///
/// 添字は 0-based であることに注意する。
///
/// ```
/// use library::binary_indexed_tree::BinaryIndexedTree;
///
/// let mut bit: BinaryIndexedTree<u32> = BinaryIndexedTree::new(5);
/// bit.add(0, 1);
/// bit.add(2, 100);
/// bit.add(4, 10000);
/// assert_eq!(&format!("{}", bit), "1 0 100 0 10000");
///
/// assert_eq!(bit.sum(0..2), 1);
/// assert_eq!(bit.sum(0..=2), 101);
/// assert_eq!(bit.sum(2..), 10100);
/// assert_eq!(bit.sum(..=4), 10101);
///
/// bit.add(1, 10);
/// assert_eq!(&format!("{}", bit), "1 10 100 0 10000");
/// assert_eq!(bit.sum(1..4), 110);
/// assert_eq!(bit.sum(0..=1), 11);
/// ```
///
/// ## 計算量
///
/// 区間和を計算したい型 `T` の空間計算量が $`O(1)`$ であり、加法が $`O(1)`$ で行えることを仮定する。
///
/// | 関数 | 概要 | 計算量 |
/// | --- | --- | --- |
/// | `new(size)` | `[0; size]` で初期化する | $`O(\text{size})`$ |
/// | `self.add(i, w)` | $`i`$ 番目の要素に `w` を足す | $`O(\log(\text{self.size}))`$ |
/// | `self.sum(range)` | `range` 内の要素の総和を求める | $`O(\log(\text{self.size}))`$ |
///
/// ## Verified problems
///
/// * [Static Range Sum](../../src/lc_static_range_sum_02/lc_static_range_sum_02.rs.html)
///
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BinaryIndexedTree<T> {
    tree: Vec<T>,
    /// 要素数を表す。
    pub size: usize,
}

impl<T: Default + Clone + Copy + std::ops::AddAssign + std::ops::Sub<Output = T>>
    BinaryIndexedTree<T>
{
    /// 要素数が `size` で各要素が `T::default()` である `BinaruIndexedTree<T>` を生成する。
    pub fn new(size: usize) -> Self {
        return Self {
            tree: vec![T::default(); size + 1],
            size,
        };
    }

    /// $`i`$ 番目の要素に $`w`$ を加算する。
    pub fn add(&mut self, i: usize, w: T) {
        assert!(i < self.size);
        self._add(i + 1, w);
    }

    /// $`\displaystyle \sum_{0 \leq j \leq i} \text{self} \lbrack j \rbrack`$ を計算する。
    pub fn prefix_sum(&self, i: usize) -> T {
        assert!(i < self.size);
        self._sum(i + 1)
    }

    /// $`\displaystyle \sum_{i \in \text{range}} \text{self} \lbrack i \rbrack`$ を計算する。
    pub fn sum<R: std::ops::RangeBounds<usize>>(&self, range: R) -> T {
        let left = match range.start_bound() {
            std::ops::Bound::Included(&l) => l,
            std::ops::Bound::Excluded(&l) => l + 1,
            std::ops::Bound::Unbounded => 0,
        };

        let right = match range.end_bound() {
            std::ops::Bound::Included(&r) => r,
            std::ops::Bound::Excluded(&r) => r - 1,
            std::ops::Bound::Unbounded => self.tree.len() - 2,
        };

        if left == 0 {
            return self.prefix_sum(right);
        } else {
            return self.prefix_sum(right) - self.prefix_sum(left - 1);
        }
    }

    fn _add(&mut self, mut i: usize, w: T) {
        while i < self.tree.len() {
            self.tree[i] += w;
            i += i & i.wrapping_neg();
        }
    }

    fn _sum(&self, mut i: usize) -> T {
        let mut ret = T::default();
        while i > 0 {
            ret += self.tree[i];
            i -= i & i.wrapping_neg();
        }
        return ret;
    }
}

impl<
        T: Default
            + Clone
            + Copy
            + std::ops::AddAssign
            + std::ops::Sub<Output = T>
            + std::fmt::Display,
    > std::fmt::Display for BinaryIndexedTree<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            (0..self.size)
                .map(|i| self.sum(i..=i))
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}
