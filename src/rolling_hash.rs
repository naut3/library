/// 文字列をハッシュ化する
///
/// ## Examples
///
/// `RollingHash::from(s)` で `s` の部分文字列をハッシュ化する事前計算を行う。このとき、事前に `STR_BASE` と `BASE` を指定する必要がある。  
/// * `STR_BASE` は文字をハッシュ化する際の基準になる。`s` に含まれる文字の中で最もUnicode scalar valueが小さいもの以下のvalueに相当する文字を与えればよい。
/// * `BASE` はハッシュの基数になる。
///
/// ```
/// use library::rolling_hash::RollingHash;
///
/// let s = "mississippi".chars().collect::<Vec<_>>();
/// let rh: RollingHash<'a', 100> = RollingHash::from(&s);
///
/// assert_eq!(rh.hash(1..=4), rh.hash(4..=7));
/// assert_ne!(rh.hash(8..), rh.hash(..3));
/// assert_eq!(rh.hash(..), rh.hash(..));
/// ```
///
/// ## 計算量
///
/// | 関数 | 概要 | 計算量 |
/// | --- | --- | --- |
/// | `from(s)` | 文字列 `s` の部分文字列のハッシュ値を計算するための事前計算を行う | $`O(\lvert \text{s} \rvert)`$ |
/// | `self.hash(range)` | `range` の範囲の部分文字列のハッシュ値を求める | $`O(1)`$ |
///
/// ## Verified problems
///
/// * [Naive String Search](../../src/aoj_alds1_14_a/aoj_alds1_14_a.rs.html)
/// * [String Search](../../src/aoj_alds1_14_b/aoj_alds1_14_b.rs.html)
///

pub struct RollingHash<const STR_BASE: char, const BASE: u64> {
    hash: Vec<u64>,
    pow: Vec<u64>,
}

impl<const STR_BASE: char, const BASE: u64> RollingHash<STR_BASE, BASE> {
    const MOD: u64 = (1_u64 << 61) - 1;
    const MASK_30: u64 = (1_u64 << 30) - 1;
    const MASK_31: u64 = (1_u64 << 31) - 1;
    const MASK_61: u64 = (1_u64 << 61) - 1;

    fn mul(a: u64, b: u64) -> u64 {
        let au = a >> 31;
        let ad = a & Self::MASK_31;
        let bu = b >> 31;
        let bd = b & Self::MASK_31;
        let mid = ad * bu + au * bd;
        let midu = mid >> 30;
        let midd = mid & Self::MASK_30;

        Self::cmod(au * bu * 2 + midu + (midd << 31) + ad * bd)
    }

    fn cmod(x: u64) -> u64 {
        let xu = x >> 61;
        let xd = x & Self::MASK_61;
        let ret = xu + xd;
        if ret >= Self::MOD {
            ret - Self::MOD
        } else {
            ret
        }
    }

    /// 文字列 `s` の部分文字列のハッシュ値を計算するための事前計算を行う
    pub fn from(s: &[char]) -> Self {
        let length = s.len();

        let mut hash = vec![0];
        let mut pow = vec![1];

        for i in 0..length {
            hash.push(Self::cmod(Self::mul(hash[i], BASE)) + s[i] as u64 + 1 - STR_BASE as u64);
            pow.push(Self::cmod(Self::mul(pow[i], BASE)));
        }

        Self {
            hash: hash,
            pow: pow,
        }
    }

    fn _h(&self, l: usize, r: usize) -> u64 {
        Self::cmod(self.hash[r] + Self::MOD * 4 - Self::mul(self.hash[l], self.pow[r - l]))
    }

    /// `range` が指定した部分文字列のハッシュ値を計算する
    pub fn hash<R: std::ops::RangeBounds<usize>>(&self, range: R) -> u64 {
        let left = match range.start_bound() {
            std::ops::Bound::Included(&l) => l,
            std::ops::Bound::Excluded(&l) => l + 1,
            std::ops::Bound::Unbounded => 0,
        };

        let right = match range.end_bound() {
            std::ops::Bound::Included(&r) => r + 1,
            std::ops::Bound::Excluded(&r) => r,
            std::ops::Bound::Unbounded => self.hash.len() - 1,
        };

        self._h(left, right)
    }
}
