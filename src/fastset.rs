/// 整数の集合を管理することに特化した順序付き集合  
///
/// <https://maspypy.com/library-checker-predecessor-problem> の <https://judge.yosupo.jp/submission/206116> を参考にした。
///
/// できること
/// * 値の挿入
/// * 値の削除
/// * ある値が含まれているかを検索する
/// * ある値以上の値で最も小さい値を検索する
/// * ある値以下の値で最も大きい値を検索する
///
/// ## Examples
///
/// ```
/// use library::fastset::FastSet;
///
/// let mut set = FastSet::new(1000);
///
/// assert!(!set.contains(0));
/// set.insert(0);
/// assert!(set.contains(0));
///
/// set.insert(999);
/// assert!(set.contains(999));
///
/// assert_eq!(set.next(100), Some(999));
/// set.remove(999);
/// assert_eq!(set.next(100), None);
/// ```
///
/// ## 計算量
///
/// 内部では、64分木を構築している。そのため、空間計算量、時間計算量共に二分探索木と変わらない。
///
/// | 関数 | 概要 | 計算量 |
/// | --- | --- | --- |
/// | `new(size)` | $`0, 1, \dots, \text{size} - 1`$ を保持することができる空の集合を生成する | $`O(\text{size})`$ |
/// | `self.insert(i)` | $`i`$ を追加する | $`O(\log(\text{self.size}))`$ |
/// | `self.remove(i)` | $`i`$ を削除する | $`O(\log(\text{self.size}))`$ |
/// | `self.contains(i)` | $`i`$ が含まれているかを検索する  | $`O(\log(\text{self.size}))`$ |
/// | `self.next(i)` | $`i`$ 以上の要素で最小の要素を検索する  | $`O(\log(\text{self.size}))`$ |
/// | `self.prev(i)` | $`i`$ 以上の要素で最小の要素を検索する  | $`O(\log(\text{self.size}))`$ |
///
/// ## Verified problems
///
/// * [Predecessor Problem](../../src/lc_predecessor_problem_03/lc_predecessor_problem_03.rs.html)
///
pub struct FastSet {
    tree: Vec<usize>,
    ptr: Vec<u32>,
    size: usize,
    height: usize,
}

impl FastSet {
    const BIT_LENGTH: usize = usize::BITS as usize;

    /// 大きさ $`size`$ の `FastSet` を生成する
    pub fn new(mut size: usize) -> Self {
        let origin_size = size;

        let mut length = 0;
        let mut ptr = vec![0u32];

        loop {
            let nxt_size = (size + Self::BIT_LENGTH - 1) / Self::BIT_LENGTH;
            length += nxt_size;
            ptr.push(length as u32);
            size = nxt_size;
            if size <= 1 {
                break;
            }
        }

        let tree = vec![0; length];
        let height = ptr.len() - 1;

        eprintln!(
            "{} | {}",
            tree.len(),
            ptr.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );

        Self {
            tree,
            ptr,
            size: origin_size,
            height,
        }
    }

    /// $`i`$ を追加する
    pub fn insert(&mut self, mut i: usize) {
        assert!(i < self.size);
        for h in 0..self.height {
            self.tree[self.ptr[h] as usize + i / Self::BIT_LENGTH] |= 1 << (i % Self::BIT_LENGTH);
            i /= Self::BIT_LENGTH;
        }
    }

    /// $`i`$ を削除する
    pub fn remove(&mut self, mut i: usize) {
        assert!(i < self.size);
        let mut x = 0usize;
        for h in 0..self.height {
            self.tree[self.ptr[h] as usize + i / Self::BIT_LENGTH] &=
                !(1usize << (i % Self::BIT_LENGTH));
            self.tree[self.ptr[h] as usize + i / Self::BIT_LENGTH] |= x << (i % Self::BIT_LENGTH);
            x = (self.tree[self.ptr[h] as usize + i / Self::BIT_LENGTH] != 0) as usize;
            i /= Self::BIT_LENGTH;
        }
    }

    /// $`i`$ を含んでいるかを検索する
    pub fn contains(&self, i: usize) -> bool {
        assert!(i < self.size);
        ((self.tree[i / Self::BIT_LENGTH] >> (i % Self::BIT_LENGTH)) & 1) == 1
    }

    /// $`i`$ 以上の要素で最小のものを検索する
    pub fn next(&self, mut i: usize) -> Option<usize> {
        assert!(i < self.size);
        for h in 0..self.height {
            if i / Self::BIT_LENGTH == (self.ptr[h + 1] - self.ptr[h]) as usize {
                break;
            }

            let d =
                self.tree[self.ptr[h] as usize + i / Self::BIT_LENGTH] >> (i % Self::BIT_LENGTH);

            if d == 0 {
                i = i / Self::BIT_LENGTH + 1;
                continue;
            }

            i += d.trailing_zeros() as usize;

            for g in (0..h).rev() {
                i *= Self::BIT_LENGTH;
                i += (self.tree[self.ptr[g] as usize + i / Self::BIT_LENGTH]).trailing_zeros()
                    as usize;
            }

            return Some(i);
        }

        return None;
    }

    /// $`i`$ 以下の要素で最大のものを検索する
    pub fn prev(&self, mut i: usize) -> Option<usize> {
        for h in 0..self.height {
            let d = self.tree[self.ptr[h] as usize + i / Self::BIT_LENGTH]
                << (Self::BIT_LENGTH - 1 - i % Self::BIT_LENGTH);

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
                    - (self.tree[self.ptr[g] as usize + i / Self::BIT_LENGTH]).leading_zeros()
                        as usize;
            }

            return Some(i);
        }

        return None;
    }
}

impl Default for FastSet {
    /// 大きさを $`2^{24}`$ に設定した `FastSet`を生成する  
    /// 64分木の構築を行わないので、その分少しだけ高速になっている
    fn default() -> Self {
        Self {
            tree: vec![0; 266305],
            ptr: vec![0, 262144, 266240, 266304, 266305],
            size: 1 << 24,
            height: 4,
        }
    }
}
