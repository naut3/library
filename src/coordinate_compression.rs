/// 座標圧縮を行う  
///
/// 長さ $`N`$ の列 $`A`$ を座標圧縮したとする。  
/// その上でできることの例は、以下の通り。  
/// * $`v`$ が $`A`$ に含まれていたかを調べる
/// * $`A`$ に含まれている要素で $`k`$ 番目に大きい要素を調べる
/// * $`A`$ に含まれている要素で、$`v`$ 以上の要素の内最小の要素を調べる(添字でもよい)
/// * $`A`$ に含まれている要素で、$`v`$ 以下の要素の内最大の要素を調べる(添字でもよい)
///
/// ## Examples
///
/// ```
/// use library::coordinate_compression::coordinate_compression;
///
/// let a = [100, 10, 1, 10000u32, 1000, 10, 100, 1000];
/// let cc = coordinate_compression(&a);
///
/// assert_eq!(cc[0], 1);
/// assert_eq!(cc[2], 100);
///
/// assert_eq!(cc.next(55), Some(100));
/// assert_eq!(cc.prev(2000), Some(1000));
///
/// assert_eq!(cc.next(100000), None);
/// assert_eq!(cc.prev(0), None);
///
/// assert_eq!(cc.index(10), Some(1));
/// assert_eq!(cc.next_index(101), Some(3));
/// assert_eq!(cc.prev_index(99), Some(1));
/// ```
///
/// ## 計算量
///
/// 長さ $`N`$ の列 `array` を座標圧縮する状況を考える。また、`array` の要素のなす集合は全順序集合であって、空間計算量が $`O(1)`$ であるとする。
///
/// | 関数 | 概要 | 計算量 |
/// | --- | --- | --- |
/// | `from(array)` | `array` を座標圧縮する | $`O(N)`$ |
/// | `self.contains(v)` | $`v`$ を含んでいるかを検索する | $`O(\log(N))`$ |
/// | `self.next(v)` | $`v`$ 以上の最小の要素を検索する | $`O(\log(N))`$ |
/// | `self.prev(v)` | $`v`$ 以下の最大の要素を検索する | $`O(\log(N))`$ |
///
pub struct CoordinateCompress<T> {
    values: Vec<T>,
    pub length: usize,
}

impl<T: std::cmp::Ord + Copy> CoordinateCompress<T> {
    /// 列 `array` を座標圧縮する
    pub fn from(array: &[T]) -> Self {
        coordinate_compression(array)
    }

    /// $`v`$ 以上の要素で最小のものがあれば、それを返す  
    /// 要素が存在しない場合、`None` を返す
    pub fn next(&self, v: T) -> Option<T> {
        if self.values.is_empty() {
            return None;
        }

        if self.values[self.length - 1] < v {
            return None;
        }

        if self.values[0] >= v {
            return Some(self.values[0]);
        }

        // この時点で、0 番目は v 未満で、最後の要素は v 以上であることが確定する
        let mut ng = 0;
        let mut ok = self.length - 1;

        while ok - ng > 1 {
            let m = ng + (ok - ng) / 2;

            if self.values[m] >= v {
                ok = m;
            } else {
                ng = m;
            }
        }

        Some(self.values[ok])
    }

    /// $`v`$ 以上の要素で最小のものがあれば、その添字を返す  
    /// 要素が存在しない場合、`None` を返す
    pub fn next_index(&self, v: T) -> Option<usize> {
        if self.values.is_empty() {
            return None;
        }

        if self.values[self.length - 1] < v {
            return None;
        }

        if self.values[0] >= v {
            return Some(0);
        }

        // この時点で、0 番目は v 未満で、最後の要素は v 以上であることが確定する
        let mut ng = 0;
        let mut ok = self.length - 1;

        while ok - ng > 1 {
            let m = ng + (ok - ng) / 2;

            if self.values[m] >= v {
                ok = m;
            } else {
                ng = m;
            }
        }

        Some(ok)
    }

    /// $`v`$ 以下の要素で最大のものがあれば、それを返す  
    /// 要素が存在しない場合、`None` を返す
    pub fn prev(&self, v: T) -> Option<T> {
        if self.values.is_empty() {
            return None;
        }

        if self.values[0] > v {
            return None;
        }

        if self.values[self.length - 1] <= v {
            return Some(self.values[self.length - 1]);
        }

        // この時点で、0 番目は v 以下で、最後の要素は v より大きいことが確定する
        let mut ok = 0;
        let mut ng = self.length - 1;

        while ng - ok > 1 {
            let m = ok + (ng - ok) / 2;

            if self.values[m] <= v {
                ok = m;
            } else {
                ng = m;
            }
        }

        Some(self.values[ok])
    }

    /// $`v`$ 以下の要素で最大のものがあれば、その添字を返す  
    /// 要素が存在しない場合、`None` を返す
    pub fn prev_index(&self, v: T) -> Option<usize> {
        if self.values.is_empty() {
            return None;
        }

        if self.values[0] > v {
            return None;
        }

        if self.values[self.length - 1] <= v {
            if self.values[self.length - 1] == v {
                return Some(self.length - 1);
            } else {
                return None;
            }
        }

        // この時点で、0 番目は v 以下で、最後の要素は v より大きいことが確定する
        let mut ok = 0;
        let mut ng = self.length - 1;

        while ng - ok > 1 {
            let m = ok + (ng - ok) / 2;

            if self.values[m] <= v {
                ok = m;
            } else {
                ng = m;
            }
        }

        Some(ok)
    }

    /// $`v`$ を含んでいるかを検索する
    pub fn contains(&self, v: T) -> bool {
        self.next(v) == Some(v)
    }

    /// $`v`$ が含まれている場合何番目に大きい要素であるかを検索する   
    /// 含まれていない場合 `None` を返す
    pub fn index(&self, v: T) -> Option<usize> {
        if self.values.is_empty() {
            return None;
        }

        if self.values[self.length - 1] < v {
            return None;
        }

        if self.values[0] >= v {
            if self.values[0] == v {
                return Some(0);
            } else {
                return None;
            }
        }

        // この時点で、0 番目は v 未満で、最後の要素は v 以上であることが確定する
        let mut ng = 0;
        let mut ok = self.length - 1;

        while ok - ng > 1 {
            let m = ng + (ok - ng) / 2;

            if self.values[m] >= v {
                ok = m;
            } else {
                ng = m;
            }
        }

        if self.values[ok] == v {
            return Some(ok);
        } else {
            return None;
        }
    }

    /// 最小の要素  
    /// 要素数が $`1`$ 以上であることが仮定される
    pub fn min(&self) -> T {
        assert!(self.length > 0);
        self.values[0]
    }

    /// 最大の要素  
    /// 要素数が $`1`$ 以上であることが仮定される
    pub fn max(&self) -> T {
        assert!(self.length > 0);
        self.values[self.length - 1]
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

impl<T> std::ops::Index<usize> for CoordinateCompress<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl<T: std::fmt::Display> std::fmt::Display for CoordinateCompress<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.values
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

pub fn coordinate_compression<T: std::cmp::Ord + Copy>(values: &[T]) -> CoordinateCompress<T> {
    let mut s = values.iter().cloned().collect::<Vec<_>>();
    s.sort_unstable();
    s.dedup();
    let length = s.len();
    CoordinateCompress { values: s, length }
}
