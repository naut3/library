pub struct CumulativeSum<T> {
    size: usize,
    prefix_sum: Vec<T>,
}

impl<T: std::ops::Add<Output = T> + Default + Clone + Copy> CumulativeSum<T> {
    /// build cumulative sum from `array`
    pub fn from(array: &[T]) -> Self {
        let size = array.len();
        let mut prefix_sum = vec![T::default(); size + 1];

        for i in 0..size {
            prefix_sum[i + 1] = prefix_sum[i] + array[i];
        }

        Self { size, prefix_sum }
    }

    /// a_0 + a_1 + ... + a_i
    pub fn prefix_sum(&self, i: usize) -> T {
        self.prefix_sum[i + 1]
    }
}

impl<T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Default + Clone + Copy>
    CumulativeSum<T>
{
    /// Σ_{i ∈ R} a_i
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
