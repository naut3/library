/// [`WaveletMatrix`] は事前にデータ構造を構築することで、区間内の $`k`$ 番目に小さい要素や、$`a`$ 以上 $`b`$ 以下の要素の数などを高速に求めることができる
///
/// ## Examples
///
/// 以下は、[`WaveletMatrix`] を構築して、`quantile`, `range_freq` 系のクエリに応える例である。
///
/// ```
/// use library::wavelet_matrix::WaveletMatrix;
///
/// let wm: WaveletMatrix<()> = WaveletMatrix::from(&[3, 1, 4, 1, 5, 9], 4);
///
/// assert_eq!(wm.access(2), 4);
/// assert_eq!(wm.access(4), 5);
///
/// // 区間 [2, 5) で 0, 1, 2 番目に小さい要素
/// // [4, 1, 5] を整列して [1, 4, 5]
/// assert_eq!(
///     [
///         wm.quantile(2, 5, 0),
///         wm.quantile(2, 5, 1),
///         wm.quantile(2, 5, 2)
///     ],
///     [1, 4, 5]
/// );
///
/// assert_eq!(wm.range_freq(0, 4, 2), 2); // 区間 [0, 4) で 2 未満の要素 -> 1 が 2 個
/// assert_eq!(wm.range_freq(2, 6, 5), 2); // 区間 [2, 6) で 5 未満の要素 -> 1, 4 が 1 個
/// ```
///
/// また、[`WaveletMatrix`] を拡張したものを利用して、条件がついた区間和を計算することができる。次にその例を示す。
///
/// ```
/// use library::wavelet_matrix::WaveletMatrix;
///
/// let wm = WaveletMatrix::from_weighted(
///     &[
///         (9, 1u32),
///         (9, 10),
///         (8, 100),
///         (2, 1_000),
///         (4, 10_000),
///         (4, 100_000),
///         (3, 1_000_000),
///         (5, 10_000_000),
///         (3, 100_000_000),
///     ],
///     4,
/// );
///
/// assert_eq!(wm.range_sum(1, 5, 9), 11_100); // 区間 [1, 5) で 9 未満の要素についた重みの和
/// assert_eq!(wm.range_sum(2, 7, 4), 1_001_000); // 区間 [2, 7) で 4 未満の要素についた重みの和
/// ```
///
/// ## 計算量
///
/// \[TODO\] word-RAM としての解析を書くべきだが、面倒なので後回しにする
///
/// ## Verified problems
///
/// * [Static Range Sum](../../src/lc_static_range_sum_03/lc_static_range_sum_03.rs.html)
/// * [Range Kth Smallest](../../src/lc_range_kth_smallest/lc_range_kth_smallest.rs.html)
///
pub struct WaveletMatrix<T> {
    bvs: Vec<BitVector>,
    length: usize,
    height: usize,
    cums: Vec<Vec<T>>,
}

impl WaveletMatrix<()> {
    /// 重み付きでない WaveletMatrix を構築する
    pub fn from(array: &[u64], height: usize) -> Self {
        let mut bvs = vec![];
        let mut array = array.to_vec();

        for i in (0..height).rev() {
            // i bit 目で安定ソートする
            let mut a0 = vec![];
            let mut a1 = vec![];
            let mut bv = vec![];

            for (j, &a) in array.iter().enumerate() {
                if j % 64 == 0 {
                    bv.push(0);
                }

                bv[j / 64] |= ((a >> i) & 1) << (j % 64);

                if (a >> i) & 1 == 1 {
                    a1.push(a);
                } else {
                    a0.push(a);
                }
            }

            bvs.push(BitVector::from(&bv));
            a0.append(&mut a1);

            array = a0;
        }

        bvs.reverse();

        Self {
            bvs,
            length: array.len(),
            height,
            cums: vec![],
        }
    }
}

impl<T> WaveletMatrix<T> {
    /// i 番目の要素の値を取得する
    pub fn access(&self, mut i: usize) -> u64 {
        // 上のbitから順番に位置を変更しながら走査すればよい
        let mut ret = 0u64;

        for j in (0..self.height).rev() {
            if self.bvs[j].access(i) {
                ret |= 1 << j;

                // 0の数 + 1の数
                i = self.bvs[j].rank(i, true) as usize
                    + self.bvs[j].rank(self.length - 1, false) as usize
                    - 1;
            } else {
                i = self.bvs[j].rank(i, false) as usize - 1;
            }
        }

        return ret;
    }

    /// [l, r) の中で k 番目に小さい値を求める (0 <= k)
    pub fn quantile(&self, mut l: usize, mut r: usize, mut k: usize) -> u64 {
        let mut ret = 0u64;

        for j in (0..self.height).rev() {
            let l0 = if l > 0 {
                self.bvs[j].rank(l - 1, false)
            } else {
                0
            };
            let r0 = if r > 0 {
                self.bvs[j].rank(r - 1, false)
            } else {
                0
            };

            if k as u32 + l0 < r0 {
                l = l0 as usize;
                r = r0 as usize;
            } else {
                ret |= 1 << j;
                k -= r0 as usize - l0 as usize;
                let count_zeros = self.bvs[j].rank(self.length - 1, false);
                l += (count_zeros - l0) as usize;
                r += (count_zeros - r0) as usize;
            }
        }

        return ret;
    }

    /// [l, r) で upper 未満の要素の数を求める
    pub fn range_freq(&self, mut l: usize, mut r: usize, upper: u64) -> u64 {
        let mut ret = 0u64;

        for j in (0..self.height).rev() {
            let l0 = if l > 0 {
                self.bvs[j].rank(l - 1, false)
            } else {
                0
            };
            let r0 = if r > 0 {
                self.bvs[j].rank(r - 1, false)
            } else {
                0
            };

            if (upper >> j) & 1 == 1 {
                ret += (r0 - l0) as u64;
                let count_zeros = self.bvs[j].rank(self.length - 1, false);

                l += (count_zeros - l0) as usize;
                r += (count_zeros - r0) as usize;
            } else {
                l = l0 as usize;
                r = r0 as usize;
            }
        }

        ret
    }
}

impl WaveletMatrix<u64> {
    /// 自身の値を使った総和系クエリを利用する場合のWavelet Matrixを構築する
    pub fn from_weighted_own(array: &[u64], height: usize) -> Self {
        let mut bvs = vec![];
        let mut cums = vec![];
        let mut array = array.to_vec();

        for i in (0..height).rev() {
            // i bit 目で安定ソートする
            let mut a0 = vec![];
            let mut a1 = vec![];
            let mut bv = vec![];

            for (j, &a) in array.iter().enumerate() {
                if j % 64 == 0 {
                    bv.push(0);
                }

                bv[j / 64] |= ((a >> i) & 1) << (j % 64);

                if (a >> i) & 1 == 1 {
                    a1.push(a);
                } else {
                    a0.push(a);
                }
            }

            bvs.push(BitVector::from(&bv));
            a0.append(&mut a1);

            let mut cs = vec![0];

            for j in 0..array.len() {
                cs.push(cs[j] + a0[j]);
            }

            cums.push(cs);
            array = a0;
        }

        bvs.reverse();
        cums.reverse();

        Self {
            bvs,
            length: array.len(),
            height,
            cums,
        }
    }
}

impl<T: Default + std::ops::Add<Output = T> + Clone + Copy> WaveletMatrix<T> {
    /// 自身の値を使わない総和系クエリを利用する場合のWavelet Matrixを構築する
    pub fn from_weighted(array: &[(u64, T)], height: usize) -> Self {
        let mut bvs = vec![];
        let mut cums = vec![];
        let mut array = array.to_vec();

        for i in (0..height).rev() {
            let mut a0 = vec![];
            let mut a1 = vec![];
            let mut bv = vec![];

            for (j, &a) in array.iter().enumerate() {
                if j % 64 == 0 {
                    bv.push(0);
                }

                bv[j / 64] |= ((a.0 >> i) & 1) << (j % 64);

                if (a.0 >> i) & 1 == 1 {
                    a1.push(a);
                } else {
                    a0.push(a);
                }
            }

            bvs.push(BitVector::from(&bv));
            a0.append(&mut a1);

            let mut cs = vec![T::default()];

            for j in 0..array.len() {
                cs.push(cs[j] + a0[j].1);
            }

            cums.push(cs);
            array = a0;
        }

        bvs.reverse();
        cums.reverse();

        Self {
            bvs,
            length: array.len(),
            height,
            cums,
        }
    }
}

impl<T: Default + std::ops::Add<Output = T> + Clone + Copy + std::ops::Sub<Output = T>>
    WaveletMatrix<T>
{
    /// [l, r) で upper 未満の要素の総和を求める
    pub fn range_sum(&self, mut l: usize, mut r: usize, upper: u64) -> T {
        let mut ret = T::default();

        for j in (0..self.height).rev() {
            let l0 = if l > 0 {
                self.bvs[j].rank(l - 1, false)
            } else {
                0
            };
            let r0 = if r > 0 {
                self.bvs[j].rank(r - 1, false)
            } else {
                0
            };

            if (upper >> j) & 1 == 1 {
                ret = ret + self.cums[j][r0 as usize] - self.cums[j][l0 as usize];
                let count_zeros = self.bvs[j].rank(self.length - 1, false);
                l += (count_zeros - l0) as usize;
                r += (count_zeros - r0) as usize;
            } else {
                l = l0 as usize;
                r = r0 as usize;
            }
        }

        ret
    }

    /// [l, r) の要素の総和を求める
    pub fn sum(&self, mut l: usize, mut r: usize) -> T {
        let mut ret = T::default();

        for j in (0..self.height).rev() {
            let l0 = if l > 0 {
                self.bvs[j].rank(l - 1, false)
            } else {
                0
            };
            let r0 = if r > 0 {
                self.bvs[j].rank(r - 1, false)
            } else {
                0
            };

            ret = ret + self.cums[j][r0 as usize] - self.cums[j][l0 as usize];
            let count_zeros = self.bvs[j].rank(self.length - 1, false);
            l += (count_zeros - l0) as usize;
            r += (count_zeros - r0) as usize;
        }

        ret
    }
}

impl<T> std::fmt::Display for WaveletMatrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..64 {
            let _ = writeln!(f, "{}", self.bvs[i]);
        }
        Ok(())
    }
}

pub struct BitVector {
    row: Vec<u64>,
    cs: Vec<u32>,
}

impl BitVector {
    pub fn from(array: &[u64]) -> Self {
        let mut cs = vec![0];

        for (i, a) in array.iter().enumerate() {
            cs.push(cs[i] + a.count_ones());
        }

        return Self {
            row: array.to_vec(),
            cs,
        };
    }

    /// i bit 目の値を返す (i は 0 以上)
    fn access(&self, i: usize) -> bool {
        (self.row[i / 64] >> (i % 64)) & 1 == 1
    }

    /// i bit 目までの b の数を数える (0 bit 目の存在に注意)
    fn rank(&self, i: usize, b: bool) -> u32 {
        if i % 64 == 63 {
            let c = self.cs[i / 64 + 1];

            if b {
                c
            } else {
                i as u32 + 1 - c
            }
        } else {
            let c = self.cs[i / 64];
            let dc = (self.row[i / 64] & ((1 << ((i % 64) + 1)) - 1)).count_ones();

            if b {
                c + dc
            } else {
                i as u32 + 1 - (c + dc)
            }
        }
    }

    /// [TODO] とりあえず使わないので後回しにする
    #[allow(unused)]
    fn select(&self, i: usize, b: bool) -> usize {
        todo!()
    }
}

impl std::fmt::Display for BitVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.row
                .iter()
                .map(|x| format!("{:>064b}", x).chars().rev().collect::<String>())
                .collect::<Vec<_>>()
                .join("")
        )
    }
}
