/// [`CumulativeSum`] は、事前にデータ構造を構築することで、区間和を高速に求めることができる。
///
/// ## Examples
///
/// 添字は 0-based であることに注意する。
///
/// ```
/// use library::cumulative_sum::CumulativeSum;
///
/// let cs = CumulativeSum::from(&[1, 10, 100, 1000, 10000]);
///
/// assert_eq!(cs.sum(1..4), 1110);
/// assert_eq!(cs.sum(0..=2), 111);
/// assert_eq!(cs.sum(3..), 11000);
/// assert_eq!(cs.sum(..4), 1111);
/// ```
///
/// ## 計算量
///
/// `T` の空間計算量が $`O(1)`$ であり、加法が $`O(1)`$ で行えることを仮定する。
///
/// | 関数 | 概要 | 計算量 |
/// | --- | --- | --- |
/// | `from(array)` | `array` をもとに累積和を生成する | $`O(\lvert \text{array} \rvert)`$ |
/// | `self.sum(range)` | `range` 内の要素の総和を求める | $`O(1)`$ |
///
/// ## Verified problems
///
/// * [Static Range Sum](../../src/lc_static_range_sum_01/lc_static_range_sum_01.rs.html)
///
pub struct CumulativeSum<T> {
    size: usize,
    prefix_sum: Vec<T>,
}

impl<T: std::ops::Add<Output = T> + Default + Clone + Copy> CumulativeSum<T> {
    /// `array` から累積和を構築する
    pub fn from(array: &[T]) -> Self {
        let size = array.len();
        let mut prefix_sum = vec![T::default(); size + 1];

        for i in 0..size {
            prefix_sum[i + 1] = prefix_sum[i] + array[i];
        }

        Self { size, prefix_sum }
    }

    /// $`\displaystyle \sum_{j \leq i} \text{self} \lbrack j \rbrack`$ を計算する
    pub fn prefix_sum(&self, i: usize) -> T {
        self.prefix_sum[i + 1]
    }
}

impl<T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Default + Clone + Copy>
    CumulativeSum<T>
{
    /// $`\displaystyle \sum_{i \in \text{range}} \text{self} \lbrack i \rbrack`$ を計算する
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
