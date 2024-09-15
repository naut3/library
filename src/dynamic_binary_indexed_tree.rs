/// 動的 Binary Indexed Tree に使う map の型を決める  
/// FxHashMap を使いたいときは、ここを書き換える
pub type Map<K, V> = std::collections::HashMap<K, V, std::collections::hash_map::RandomState>;

/// 必要なところだけ値を持つようにした BinaryIndexedTree
///
/// ## Examples
///
/// ```
/// use library::dynamic_binary_indexed_tree::DynamicBinaryIndexedTree;
///
/// let mut bit = DynamicBinaryIndexedTree::new(1 << 32);
///
/// bit.add(1 << 30, 1);
/// bit.add(1 << 15, 10);
/// bit.add(1, 100);
///
/// assert_eq!(bit.sum(1 << 15..=1 << 30), 11);
/// assert_eq!(bit.sum(1 << 15..1 << 30), 10);
/// assert_eq!(bit.sum(1..=1 << 15), 110);
/// assert_eq!(bit.sum((1 << 15) + 1..1 << 30), 0);
/// ```
///
/// ## 計算量
///
/// 区間和を計算したい型 `T` の空間計算量が $`O(1)`$ であり、加法が $`O(1)`$ で行えることを仮定する。
///
/// | 関数 | 概要 | 計算量 |
/// | --- | --- | --- |
/// | `new(size)` | `[0; size]` で初期化する | $`O(1)`$ |
/// | `self.add(i, w)` | $`i`$ 番目の要素に `w` を足す | $`O(\log(\text{self.size}))`$ |
/// | `self.sum(range)` | `range` 内の要素の総和を求める | $`O(\log(\text{self.size}))`$ |
///
/// ## Verified problems
///
/// * [Static Range Sum](../../src/lc_static_range_sum_04/lc_static_range_sum_04.rs.html)
///
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DynamicBinaryIndexedTree<T> {
    tree: Map<usize, T>,
    pub size: usize,
}

impl<T: Default + std::ops::AddAssign + Copy> DynamicBinaryIndexedTree<T> {
    /// `[0; size]` で初期化する
    pub fn new(size: usize) -> Self {
        Self {
            tree: Map::new(),
            size,
        }
    }

    fn _add(&mut self, mut i: usize, w: T) {
        while i <= self.size {
            *self.tree.entry(i).or_insert(T::default()) += w;
            i += i & i.wrapping_neg();
        }
    }

    /// $`i`$ 番目の要素に `w` を足す
    pub fn add(&mut self, i: usize, w: T) {
        assert!(i < self.size);
        self._add(i + 1, w)
    }

    fn prefix_sum(&self, i: usize) -> T {
        assert!(i < self.size);
        self._sum(i + 1)
    }

    fn _sum(&self, mut i: usize) -> T {
        let mut r = T::default();
        while i > 0 {
            if let Some(&v) = self.tree.get(&i) {
                r += v;
            }
            i -= i & i.wrapping_neg();
        }
        r
    }
}

impl<T: Default + std::ops::AddAssign + std::ops::Sub<Output = T> + Copy>
    DynamicBinaryIndexedTree<T>
{
    /// `range` 内の要素の総和を求める
    pub fn sum<R: std::ops::RangeBounds<usize>>(&self, range: R) -> T {
        let left = match range.start_bound() {
            std::ops::Bound::Included(&l) => l,
            std::ops::Bound::Excluded(&l) => l + 1,
            std::ops::Bound::Unbounded => 0,
        };

        let right = match range.end_bound() {
            std::ops::Bound::Included(&r) => r,
            std::ops::Bound::Excluded(&r) => r - 1,
            std::ops::Bound::Unbounded => self.size - 1,
        };

        if left == 0 {
            return self.prefix_sum(right);
        } else {
            return self.prefix_sum(right) - self.prefix_sum(left - 1);
        }
    }
}
