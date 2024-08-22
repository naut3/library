pub struct BinaryIndexedTree<T> {
    tree: Vec<T>,
}

impl<T: Default + Clone + Copy + std::ops::AddAssign + std::ops::Sub<Output = T>>
    BinaryIndexedTree<T>
{
    /// self = [0; size]
    pub fn new(size: usize) -> Self {
        return Self {
            tree: vec![T::default(); size + 1],
        };
    }

    /// self[i] <- self[i] + w
    pub fn add(&mut self, i: usize, w: T) {
        self._add(i + 1, w);
    }

    /// return Σ_{j ∈ [0, i]} self[j]
    pub fn prefix_sum(&self, i: usize) -> T {
        self._sum(i + 1)
    }

    /// return Σ_{j ∈ range} self[j]
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
