/// 集合を合併する操作、2つの要素が同じ集合に含まれているかを検索する操作が行える素集合データ構造
///
/// ## Examples
///
/// ```
/// use library::unionfind::UnionFind;
///
/// let mut uf = UnionFind::new(3);
///
/// assert_eq!(uf.is_same(0, 0), true);
/// assert_eq!(uf.is_same(0, 2), false);
/// assert_eq!(uf.size(0), 1);
///
/// uf.unite(0, 2);
/// // 0 と 2 を合併したので、is_same(0, 2) = true になった
/// assert_eq!(uf.is_same(0, 2), true);
///
/// assert_eq!(uf.is_same(0, 1), false);
/// // 0 と 2 はすでに合併されているので、0, 1, 2 が同じ集合に含まれている
/// uf.unite(2, 1);
/// assert_eq!(uf.is_same(1, 0), true);
///
/// assert_eq!(uf.size(1), 3);
/// ```
///
/// ## 計算量
///
/// $`\lvert \text{self} \rvert`$ を初めに生成したときの素集合の数とする。
///
/// | 関数 | 概要 | 計算量 |
/// | --- | --- | --- |
/// | `new(size)` | $`\{ 0 \}, \{ 1 \}, \dots, \{ \text{size} - 1 \}`$ で初期化する | $`O(\text{size})`$ |
/// | `self.unite(a, b)` | $`a`$ が含まれている集合と $`b`$ が含まれている集合を合併する | $`O(\alpha(\lvert \text{self} \rvert))`$ |
/// | `self.is_same(u, v)` | $`u`$ が含まれている集合と $`v`$ が含まれている集合が同じかどうかを検索する | $`O(\alpha(\lvert \text{self} \rvert))`$ |
/// | `self.size(v)` | $`v`$ が含まれている集合の大きさを求める | $`O(\alpha(\lvert \text{self} \rvert))`$ |
///
/// ## Verified problems
///
/// * [Unionfind](../../src/lc_unionfind/lc_unionfind.rs.html)
/// * [Minimum Spanning Tree](../../src/aoj_grl_2_a/aoj_grl_2_a.rs.html)
///

pub struct UnionFind {
    data: Vec<i32>,
}

impl UnionFind {
    /// $`\{ 0 \}, \{ 1 \}, \dots, \{ \text{size} - 1 \}`$ で初期化する
    pub fn new(size: usize) -> Self {
        return Self {
            data: vec![-1; size],
        };
    }

    /// $`u`$ が含まれている集合と $`v`$ が含まれている集合が同じかどうかを検索する
    pub fn is_same(&mut self, u: usize, v: usize) -> bool {
        assert!(v < self.data.len() && u < self.data.len());
        self._find(u) == self._find(v)
    }

    /// $`a`$ が含まれている集合と $`b`$ が含まれている集合を合併する
    pub fn unite(&mut self, mut a: usize, mut b: usize) -> () {
        assert!(a < self.data.len() && b < self.data.len());
        a = self._find(a);
        b = self._find(b);

        if a == b {
            return;
        }
        if self.data[a] > self.data[b] {
            (a, b) = (b, a);
        }

        self.data[a] += self.data[b];
        self.data[b] = a as i32;
    }

    /// $`v`$ が含まれている集合の大きさを求める
    pub fn size(&mut self, mut v: usize) -> i32 {
        assert!(v < self.data.len());
        v = self._find(v);
        -self.data[v]
    }

    fn _find(&mut self, v: usize) -> usize {
        assert!(v < self.data.len());
        if self.data[v] < 0 {
            return v;
        }

        self.data[v] = self._find(self.data[v] as usize) as i32;
        return self.data[v] as usize;
    }
}
