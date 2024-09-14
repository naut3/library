use crate::algebra::Band;

/// 帯(冪等半群)に対して事前構築を行って区間積を $`O(1)`$ で計算できる
///
/// ## Examples
///
/// すでに定義されたものを利用する場合
///
/// ```
/// use library::algebra::Max;
/// use library::sparse_table::SparseTable;
///
/// let a = [100, 10, 1, 1000, 10000, 1, 100, 10];
/// let st: SparseTable<Max<u32>> = SparseTable::from(&a);
///
/// // max(a[1..=3]) = max(10, 1, 1000) = 1000
/// assert_eq!(st.prod(1..=3), 1000);
/// assert_eq!(st.prod(3..6), 10000);
/// assert_eq!(st.prod(7..), 10);
/// assert_eq!(st.prod(..3), 100);
/// ```
///
/// その場で定義したものを利用する場合
///
/// ```
/// use library::algebra::Band;
/// use library::sparse_table::SparseTable;
///
/// enum B {}
///
/// impl Band for B {
///     type S = u16;
///     fn op(lhs: &Self::S, rhs: &Self::S) -> Self::S {
///         lhs | rhs
///     }
/// }
///
/// let a = [0b1000, 0b0101, 0b0010, 0b0110, 0b0001, 0b1100, 0b1101];
/// let st: SparseTable<B> = SparseTable::from(&a);
///
/// assert_eq!(st.prod(0..2), 0b1101);
/// assert_eq!(st.prod(2..=3), 0b0110);
/// assert_eq!(st.prod(5..), 0b1101);
/// ```
///
/// ## 計算量
///
/// 帯 `B` の集合 `S` の空間計算量が $`O(1)`$ であり、二項演算が $`O(1)`$ で行えることを仮定する。  
///
/// | 関数 | 概要 | 計算量 |
/// | --- | --- | --- |
/// | `from(array)` | `array` からデータ構造を構築する | $`O(\lvert \text{array} \rvert \log(\lvert \text{array} \rvert))`$ |
/// | `self.prod(range)` | $`\displaystyle \prod_{i \in \text{range}} \text{array} \lbrack i \rbrack`$ | $`O(1)`$ |
///
/// ## Verified problems
///
/// * [Static RMQ](../../src/lc_static_rmq/lc_static_rmq.rs.html)
///
pub struct SparseTable<B: Band> {
    size: usize,
    table: Vec<B::S>,
}

impl<B: Band<S = S>, S: Clone + Copy> SparseTable<B> {
    pub fn from(array: &[S]) -> Self {
        let size = array.len();
        let height = size.next_power_of_two().trailing_zeros() as usize;

        let mut table = array.to_vec();

        for _ in 1..height {
            table.append(&mut array.to_vec());
        }

        let flatten = |y: usize, x: usize| y * size + x;

        for h in 1..height {
            for i in 0..size {
                if i + (1 << (h - 1)) - 1 >= size {
                    break;
                }

                table[flatten(h, i)] = B::op(
                    &table[flatten(h - 1, i)],
                    &table[flatten(h - 1, i + (1 << (h - 1)))],
                );
            }
        }

        return Self { size, table };
    }

    fn _prod(&self, l: usize, r: usize) -> S {
        assert!(l < self.size && r <= self.size);

        if r == l + 1 {
            return self.table[l];
        }

        let length = r - l;
        let w = (length.next_power_of_two()) >> 1;
        let h = ((length.next_power_of_two()) >> 1).trailing_zeros() as usize;

        let flatten = |y: usize, x: usize| y * self.size + x;

        B::op(&self.table[flatten(h, l)], &self.table[flatten(h, r - w)])
    }

    pub fn prod<R: std::ops::RangeBounds<usize>>(&self, range: R) -> S {
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

        self._prod(left, right)
    }
}
