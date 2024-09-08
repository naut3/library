pub type Index = u32;

pub struct Doubling {
    dp: Vec<Index>,
    size: usize,
    depth: Index,
}

impl Doubling {
    /// build doubling array from `nxt`
    pub fn build(nxt: &[Index], depth: Index) -> Self {
        let size = nxt.len();

        let mut dp = nxt.clone().to_vec();
        dp.append(&mut vec![0; size * depth as usize]);

        for d in 0..depth as usize {
            for i in 0..size {
                dp[(d + 1) * size + i] = dp[d * size + dp[d * size + i] as usize];
            }
        }

        Self { dp, size, depth }
    }

    /// return `k` next element from `src`
    pub fn next(&self, mut src: Index, k: u64) -> Index {
        assert!(k < 1 << (self.depth + 1));

        for i in 0..self.depth {
            if (k >> i) & 1 == 1 {
                src = self.dp[i as usize * self.size + src as usize];
            }
        }

        src
    }

    /// return 2 ** `k` next element from `src`
    pub fn jump_power_of_two(&self, src: Index, k: Index) -> Index {
        assert!(k <= self.depth);
        self.dp[k as usize * self.size + src as usize]
    }
}
