///
/// <https://maspypy.com/library-checker-predecessor-problem> の <https://judge.yosupo.jp/submission/206116> を参考にした。
///
pub struct FastSet {
    tree: Vec<Vec<u64>>,
    size: usize,
    height: usize,
}

impl FastSet {
    const BIT_LENGTH: usize = u64::BITS as usize;

    /// 大きさ $`size`$ の `FastSet` を生成する
    pub fn new(mut size: usize) -> Self {
        let origin_size = size;
        let mut tree = vec![];

        loop {
            let nxt_size = (size + Self::BIT_LENGTH - 1) / Self::BIT_LENGTH;
            tree.push(vec![0; nxt_size]);
            size = nxt_size;
            if size <= 1 {
                break;
            }
        }

        let height = tree.len();

        Self {
            tree,
            size: origin_size,
            height,
        }
    }

    /// $`i`$ を追加する
    pub fn insert(&mut self, mut i: usize) {
        assert!(i < self.size);
        for h in 0..self.height {
            self.tree[h][i / Self::BIT_LENGTH] |= 1 << (i % Self::BIT_LENGTH);
            i /= Self::BIT_LENGTH;
        }
    }

    /// $`i`$ を削除する
    pub fn remove(&mut self, mut i: usize) {
        assert!(i < self.size);
        let mut x = 0u64;
        for h in 0..self.height {
            self.tree[h][i / Self::BIT_LENGTH] &= !(1u64 << (i % Self::BIT_LENGTH));
            self.tree[h][i / Self::BIT_LENGTH] |= x << (i % Self::BIT_LENGTH);
            x = (self.tree[h][i / Self::BIT_LENGTH] != 0) as u64;
            i /= Self::BIT_LENGTH;
        }
    }

    /// $`i`$ を含んでいるかを検索する
    pub fn contains(&self, i: usize) -> bool {
        assert!(i < self.size);
        ((self.tree[0][i / Self::BIT_LENGTH] >> (i % Self::BIT_LENGTH)) & 1) == 1
    }

    /// $`i`$ 以上の要素で最小のものを検索する
    pub fn next(&self, mut i: usize) -> Option<usize> {
        assert!(i < self.size);
        for h in 0..self.height {
            if i / Self::BIT_LENGTH == self.tree[h].len() {
                break;
            }

            let d = self.tree[h][i / Self::BIT_LENGTH] >> (i % Self::BIT_LENGTH);

            if d == 0 {
                i = i / Self::BIT_LENGTH + 1;
                continue;
            }

            i += d.trailing_zeros() as usize;

            for g in (0..h).rev() {
                i *= Self::BIT_LENGTH;
                i += (self.tree[g][i / Self::BIT_LENGTH]).trailing_zeros() as usize;
            }

            return Some(i);
        }

        return None;
    }

    /// $`i`$ 以下の要素で最大のものを検索する
    pub fn prev(&self, mut i: usize) -> Option<usize> {
        for h in 0..self.height {
            let d =
                self.tree[h][i / Self::BIT_LENGTH] << (Self::BIT_LENGTH - 1 - i % Self::BIT_LENGTH);

            if d == 0 {
                i /= Self::BIT_LENGTH;

                if i == 0 {
                    break;
                }

                i -= 1;
                continue;
            }

            i -= d.leading_zeros() as usize;

            for g in (0..h).rev() {
                i *= Self::BIT_LENGTH;
                i += Self::BIT_LENGTH
                    - 1
                    - (self.tree[g][i / Self::BIT_LENGTH]).leading_zeros() as usize;
            }

            return Some(i);
        }

        return None;
    }
}
