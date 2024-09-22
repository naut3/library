const NONE: u32 = u32::MAX;

/// bit列で構築する Trie木
///
/// できることの例は以下の通り。
///
/// * 値の挿入、削除
/// * 値の検索
/// * 最大値、最小値の取得
/// * $`k`$ 番目に小さい要素の検索
/// * $`a`$ 以上の要素で最小の要素、$`b`$ 以下の要素で最大の要素の検索
///
/// ## Examples
///
/// bit列の長さを構築時に指定する。その後は、[`std::collections::BTreeSet`]等と同じような使用感で使うことができる。
///
/// ```
/// use library::binary_trie::MultiBinaryTrie;
///
/// let mut bt: MultiBinaryTrie<4> = MultiBinaryTrie::new();
///
/// bt.insert(0b0011);
/// bt.insert(0b0101);
/// assert!(bt.contains(0b0101));
/// assert!(!bt.contains(0b1011));
///
/// bt.delete(0b0101);
/// assert!(!bt.contains(0b0101));
///
/// bt.insert(0b1001);
/// bt.insert(0b1100);
/// bt.insert(0b1011);
/// assert_eq!(bt.max().unwrap(), 0b1100);
/// assert_eq!(bt.min().unwrap(), 0b0011);
/// assert_eq!(bt.kth_elem(1).unwrap(), 0b1001);
/// assert_eq!(bt.kth_elem(2).unwrap(), 0b1011);
/// assert_eq!(bt.lower_bound(0b1010).unwrap(), 0b1011);
/// assert_eq!(bt.upper_bound(0b1100).unwrap(), 0b1100);
/// ```
///
/// 多重集合であることに注意する。
///
/// ```
/// use library::binary_trie::MultiBinaryTrie;
///
/// let mut bt: MultiBinaryTrie<4> = MultiBinaryTrie::new();
///
/// bt.insert(0b0001);
/// assert!(bt.contains(0b0001));
///
/// bt.insert(0b0001);
/// assert_eq!(bt.count(0b0001), 2);
///
/// bt.remove(0b0001);
/// assert_eq!(bt.count(0b0001), 1);
/// ```
///
/// ## 計算量
///
/// 最初に指定された列の長さを $`D`$ とする。その上で、すべての操作は $`O(D)`$ である。
///
/// ## Verified problems
///
/// * [Binary Search](../../src/aoj_itp2_6_a/aoj_itp2_6_a.rs.html)
/// * [Predecessor Problem](../../src/lc_predecessor_problem/lc_predecessor_problem.rs.html)
/// * [Set Xor-Min](../../src/lc_set_xor_min/lc_set_xor_min.rs.html)
///
#[derive(Clone)]
pub struct MultiBinaryTrie<const D: u8> {
    tree: Vec<Node>,
}

impl<const D: u8> MultiBinaryTrie<D> {
    /// 空の BinaryTrie を生成する
    pub fn new() -> Self {
        Self {
            tree: vec![Node(NONE, NONE, 0)],
        }
    }

    /// $`x`$ を一つ追加する
    pub fn insert(&mut self, x: u64) {
        let mut ptr = 0;

        for d in (0..D).rev() {
            let b = ((x >> d) & 1) == 1;

            if b {
                if self.tree[ptr].1 == NONE {
                    let l = self.tree.len();
                    self.tree.push(Node(NONE, NONE, 0));
                    self.tree[ptr].1 = l as u32;
                }
                ptr = self.tree[ptr].1 as usize;
            } else {
                if self.tree[ptr].0 == NONE {
                    let l = self.tree.len();
                    self.tree.push(Node(NONE, NONE, 0));
                    self.tree[ptr].0 = l as u32;
                }
                ptr = self.tree[ptr].0 as usize;
            }

            self.tree[ptr].2 += 1;
        }
    }

    /// $`x`$ を一つ削除する
    ///
    /// 削除できる、つまり実行前に $`x`$ を含んでいる場合 `true` を返す
    /// そうでない場合 `false` を返す
    pub fn remove(&mut self, x: u64) -> bool {
        if !self.contains(x) {
            return false;
        }

        let mut ptr = 0;

        for d in (0..D).rev() {
            let b = ((x >> d) & 1) == 1;

            if b {
                ptr = self.tree[ptr].1 as usize;
            } else {
                ptr = self.tree[ptr].0 as usize;
            }

            self.tree[ptr].2 -= 1;
        }

        true
    }

    fn has_zero_node(&self, ptr: usize) -> bool {
        self.tree[ptr].0 != NONE && self.tree[self.tree[ptr].0 as usize].2 > 0
    }

    fn has_one_node(&self, ptr: usize) -> bool {
        self.tree[ptr].1 != NONE && self.tree[self.tree[ptr].1 as usize].2 > 0
    }

    /// $`x`$ が含まれているかを調べる
    pub fn contains(&self, x: u64) -> bool {
        let mut ptr = 0;

        for d in (0..D).rev() {
            let b = ((x >> d) & 1) == 1;

            if b {
                if !self.has_one_node(ptr) {
                    return false;
                }

                ptr = self.tree[ptr].1 as usize;
            } else {
                if !self.has_zero_node(ptr) {
                    return false;
                }

                ptr = self.tree[ptr].0 as usize;
            }
        }

        true
    }

    /// $`x`$ が何個含まれているかを調べる
    pub fn count(&self, x: u64) -> u32 {
        let mut ptr = 0;

        for d in (0..D).rev() {
            let b = ((x >> d) & 1) == 1;

            if b {
                if !self.has_one_node(ptr) {
                    return 0;
                }

                ptr = self.tree[ptr].1 as usize;
            } else {
                if !self.has_zero_node(ptr) {
                    return 0;
                }

                ptr = self.tree[ptr].0 as usize;
            }
        }

        self.tree[ptr].2
    }

    /// 今自身が何個の要素を含んでいるかを求める
    pub fn all_count(&self) -> u32 {
        let mut cnt = 0;

        if self.tree[0].0 != NONE {
            cnt += self.tree[self.tree[0].0 as usize].2;
        }

        if self.tree[0].1 != NONE {
            cnt += self.tree[self.tree[0].1 as usize].2;
        }

        cnt
    }

    /// 現在自身に含まれている要素で最小のものを求める
    pub fn min(&self) -> Option<u64> {
        if self.all_count() == 0 {
            return None;
        }

        let mut ptr = 0;
        let mut value = 0;

        for d in (0..D).rev() {
            if self.has_zero_node(ptr) {
                ptr = self.tree[ptr].0 as usize;
            } else if self.has_one_node(ptr) {
                ptr = self.tree[ptr].1 as usize;
                value |= 1 << d;
            } else {
                return Some(value);
            }
        }

        Some(value)
    }

    /// $`\min_{e \in \text{self}} e \text{XOR} x`$ を求める
    pub fn xor_min(&self, x: u64) -> Option<u64> {
        if self.all_count() == 0 {
            return None;
        }

        let mut ptr = 0;
        let mut value = 0;

        for d in (0..D).rev() {
            let b = ((x >> d) & 1) == 1;

            if !b {
                if self.has_zero_node(ptr) {
                    ptr = self.tree[ptr].0 as usize;
                } else if self.has_one_node(ptr) {
                    ptr = self.tree[ptr].1 as usize;
                    value |= 1 << d;
                } else {
                    return Some(value);
                }
            } else {
                if self.has_one_node(ptr) {
                    ptr = self.tree[ptr].1 as usize;
                    value |= 1 << d;
                } else if self.has_zero_node(ptr) {
                    ptr = self.tree[ptr].0 as usize;
                } else {
                    return Some(value);
                }
            }
        }

        Some(value ^ x)
    }

    /// 現在自身に含まれている要素で最大のものを求める
    pub fn max(&self) -> Option<u64> {
        if self.all_count() == 0 {
            return None;
        }

        let mut ptr = 0;
        let mut value = 0;

        for d in (0..D).rev() {
            if self.has_one_node(ptr) {
                ptr = self.tree[ptr].1 as usize;
                value |= 1 << d;
            } else if self.has_zero_node(ptr) {
                ptr = self.tree[ptr].0 as usize;
            } else {
                return Some(value);
            }
        }

        Some(value)
    }

    /// 現在自身に含まれている要素で $`k`$ 番目のものを求める ($`k \geq 0`$)
    pub fn kth_elem(&self, k: usize) -> Option<u64> {
        if self.all_count() <= k as u32 {
            return None;
        }

        let mut ptr = 0;
        let mut cnt = 0;
        let mut value = 0;

        for d in (0..D).rev() {
            if ptr as u32 == NONE || (!self.has_zero_node(ptr) && !self.has_one_node(ptr)) {
                return None;
            }

            if self.has_zero_node(ptr) {
                let dc = self.tree[self.tree[ptr].0 as usize].2;

                if cnt + dc > k as u32 {
                    ptr = self.tree[ptr].0 as usize;
                } else {
                    ptr = self.tree[ptr].1 as usize;
                    cnt += dc;
                    value |= 1 << d;
                }
            } else {
                ptr = self.tree[ptr].1 as usize;
                value |= 1 << d;
            }
        }

        return Some(value);
    }

    /// 現在自身に含まれている要素で $`x`$ 以上の値で最も小さいものを求める
    pub fn lower_bound(&self, x: u64) -> Option<u64> {
        let mut ptr = 0;
        let mut cnt = 0;

        for d in (0..D).rev() {
            let b = ((x >> d) & 1) == 1;

            if b {
                if self.has_zero_node(ptr) {
                    cnt += self.tree[self.tree[ptr].0 as usize].2;
                }

                if self.has_one_node(ptr) {
                    ptr = self.tree[ptr].1 as usize;
                } else {
                    break;
                }
            } else {
                if self.has_zero_node(ptr) {
                    ptr = self.tree[ptr].0 as usize;
                } else {
                    break;
                }
            }
        }

        self.kth_elem(cnt as usize)
    }

    /// 現在自身に含まれている要素で $`x`$ 以下の値で最も大きいものを求める
    pub fn upper_bound(&self, x: u64) -> Option<u64> {
        if self.contains(x) {
            return Some(x);
        }

        let mut ptr = 0;
        let mut cnt = 0;

        for d in (0..D).rev() {
            let b = ((x >> d) & 1) == 1;

            if b {
                if self.has_zero_node(ptr) {
                    cnt += self.tree[self.tree[ptr].0 as usize].2;
                }

                if self.has_one_node(ptr) {
                    ptr = self.tree[ptr].1 as usize;
                } else {
                    break;
                }
            } else {
                if self.has_zero_node(ptr) {
                    ptr = self.tree[ptr].0 as usize;
                } else {
                    break;
                }
            }
        }

        if cnt == 0 {
            return None;
        }

        self.kth_elem(cnt as usize - 1)
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Node(u32, u32, u32);
