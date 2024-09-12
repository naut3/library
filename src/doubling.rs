pub type Index = u32;

/// [`Doubling`] は、一個先が何かが分かっている対象の $`K`$ 個先を高速に計算するデータ構造である。
///
/// ## Usage
///
/// [`Doubling::build()`] は、`nxt`配列と`depth`を引数に取る。  
/// $`\text{nxt} \lbrack i \rbrack`$ には $`i`$ が次に移動する先を格納する。  
/// `depth` は、事前計算する量を決める。最大で$`2^{K}`$ 個先を計算する必要があるときは、$`\text{depth} \geq K`$ に設定するとよい。
///
/// より形式的には、$`S = \{ 0, 1, 2, \dots, N - 1 \}`$ という集合があり、写像 $`f \colon S \longrightarrow S`$ があるとき、このデータ構造を利用して、$`f^{K}(i)`$ を高速に計算することができる。
/// `nxt` は $`\text{nxt} \lbrack i \rbrack = f(i)`$ とすればよい。
///
/// ## Examples
///
/// ```
/// use library::doubling::Doubling;
///
/// let dbl = Doubling::build(&vec![1, 0, 3, 4, 2], 30);
///
/// assert_eq!(dbl.next(0, 1), 1);
/// assert_eq!(dbl.next(1, (1 << 30) + 1), 0);
///
/// assert_eq!(dbl.next(3, 123456), 3);
/// // 3 周期で 2^10 個先は (2^10) % 3 = ((-1) ^ 10) % 3 = 1 個先と同じである
/// assert_eq!(dbl.jump_power_of_two(2, 10), 3);
/// ```
///
/// ## 計算量
///
/// | 関数 | 概要 | 計算量 |
/// | --- | --- | --- |
/// | `build(nxt, depth)` | 事前計算を行い、データ構造を構築する | $`O(\lvert \text{nxt} \rvert \cdot \text{depth})`$ |
/// | `self.next(src, k)` | `src` から $`k`$ 回移動した先を求める | $`O(\text{self.depth})`$ |
/// | `self.jump_power_of_two(src, k)` | `src` から $`2^k`$ 回移動した先を求める | $`O(1)`$ |
///
/// ## Verified problems
///
/// * [Lowest Common Ancestor](../../src/lc_lca_01/lc_lca_01.rs.html)
///
pub struct Doubling {
    dp: Vec<Index>,
    /// 要素の数
    pub size: usize,
    /// 構築した遷移先の深さ
    pub depth: Index,
}

impl Doubling {
    /// ダブリングの配列を構築する。
    pub fn build(nxt: &[Index], depth: Index) -> Self {
        let size = nxt.len();

        let mut dp = nxt.to_vec();
        dp.append(&mut vec![0; size * depth as usize]);

        for d in 0..depth as usize {
            for i in 0..size {
                dp[(d + 1) * size + i] = dp[d * size + dp[d * size + i] as usize];
            }
        }

        Self { dp, size, depth }
    }

    /// `src` から `k` 回移動した先を求める
    pub fn next(&self, mut src: Index, k: u64) -> Index {
        assert!(k < 1 << (self.depth + 1));

        for i in 0..self.depth {
            if (k >> i) & 1 == 1 {
                src = self.dp[i as usize * self.size + src as usize];
            }
        }

        src
    }

    /// `src` から $`2^k`$ 回移動した先を求める
    pub fn jump_power_of_two(&self, src: Index, k: Index) -> Index {
        assert!(k <= self.depth);
        self.dp[k as usize * self.size + src as usize]
    }
}
