use crate::algebra::Monoid;

/// 一点更新と区間積の計算を行える `SegmentTree`
///
/// ## Examples
///
/// ```
/// use library::algebra::Add;
/// use library::segtree::SegmentTree;
///
/// let mut stree: SegmentTree<Add<i32>> = SegmentTree::new(5);
///
/// stree.insert(0, 1);
/// stree.insert(3, 1000);
/// assert_eq!(stree[0], 1);
/// assert_eq!(stree[3], 1000);
/// assert_eq!(stree.prod(0..=3), 1001);
///
/// stree.insert(2, 100);
/// assert_eq!(stree.prod(0..=3), 1101);
/// ```
///
/// ## 計算量
///
/// `SegmentTree<M>` のモノイド `M` の空間計算量が $`O(1)`$ であり、二項演算が $`O(1)`$ で行えるとする。
///
/// | 関数 | 概要 | 計算量 |
/// | --- | --- | --- |
/// | `new(size)` | 大きさ `size` で各要素が単位元の `SegmentTree` を生成する | $`O(\text{size})`$ |
/// | `self.insert(i, s)` | $`i`$ 番目の要素を $`s`$ に更新する | $`O(\log(\text{self.size}))`$ |
/// | `self.prod(range)` | `range` 内の要素の総積を求める | $`O(\log(\text{self.size}))`$ |
///
/// ## Verified Problems
///
/// * [Static Range Sum](../../src/lc_static_range_sum_05/lc_static_range_sum_05.rs.html)
///
pub struct SegmentTree<M: Monoid> {
    size: usize,
    tree: Vec<M::S>,
}

impl<M: Monoid> SegmentTree<M> {
    /// 大きさ `size` で、すべての要素が `M` の単位元である `SegmentTree<M>` を生成する
    pub fn new(size: usize) -> Self {
        Self {
            size,
            tree: vec![M::E; size << 1],
        }
    }

    /// `array` から `SegmentTree` を生成する
    pub fn from(array: &[M::S]) -> Self {
        let size = array.len();
        let tree = {
            let mut tree = vec![M::E; size];
            tree.append(&mut array.clone().to_vec());

            for i in (1..size).rev() {
                tree[i] = M::op(&tree[i << 1], &tree[i << 1 | 1]);
            }

            tree
        };

        return Self { size, tree };
    }

    /// $`i`$ 番目の要素を `s` に変更する
    pub fn insert(&mut self, mut i: usize, s: M::S) {
        assert!(i < self.size);

        i += self.size;

        self.tree[i] = s;

        while i > 1 {
            i >>= 1;
            self.tree[i] = M::op(&self.tree[i << 1], &self.tree[i << 1 | 1]);
        }
    }

    /// $`i`$ 番目の要素を返す
    pub fn get(&self, i: usize) -> M::S {
        assert!(i < self.size);
        self.tree[i + self.size].clone()
    }

    /// $`\displaystyle \prod_{i \in \text{range}} \text{self} \lbrack i \rbrack`$ を返す
    pub fn prod<R: std::ops::RangeBounds<usize>>(&self, range: R) -> M::S {
        let left = match range.start_bound() {
            std::ops::Bound::Included(&l) => l,
            std::ops::Bound::Excluded(&l) => l + 1,
            std::ops::Bound::Unbounded => 0,
        };

        let right = match range.end_bound() {
            std::ops::Bound::Included(&r) => r + 1,
            std::ops::Bound::Excluded(&r) => r,
            std::ops::Bound::Unbounded => self.size,
        };

        return self._prod(left, right);
    }

    fn _prod(&self, mut left: usize, mut right: usize) -> M::S {
        left += self.size;
        right += self.size;
        let (mut sl, mut sr) = (M::E, M::E);

        while left < right {
            if left & 1 == 1 {
                sl = M::op(&sl, &self.tree[left]);
                left += 1;
            }

            if right & 1 == 1 {
                right ^= 1;
                sr = M::op(&self.tree[right], &sr);
            }

            left >>= 1;
            right >>= 1;
        }

        return M::op(&sl, &sr);
    }
}

impl<M: Monoid> std::ops::Index<usize> for SegmentTree<M> {
    type Output = M::S;
    fn index(&self, index: usize) -> &Self::Output {
        &self.tree[index + self.size]
    }
}

impl<M: Monoid<S = S>, S: std::fmt::Display> std::fmt::Display for SegmentTree<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            (0..self.size)
                .map(|i| self.get(i))
                .collect::<Vec<_>>()
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}
