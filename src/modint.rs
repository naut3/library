use std::{fmt::*, ops::*};

/// 剰余類を扱うための構造体    
/// $`P`$ を素数として、$`\mathbb{Z} / P \mathbb{Z}`$ を扱う。
/// 
/// `ModInt<P>` と `T` の四則演算や、`T` と `ModInt<P>` の四則演算を行うときは、`ModInt<P>` に自動で変換される。  
/// `u32`, `i32`, `u64`, `i64`, `usize`, `isize` から変換できる。
/// 
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ModInt<const P: u32>(u32);

impl<const P: u32> ModInt<P> {
    /// `value` から `ModInt<P>` を生成する  
    /// $`\text{value} < P`$ であることを要求する代わりに、`ModInt<P>` への変換時に割り算を行わない。
    pub fn from_raw(value: u32) -> Self {
        assert!(value < P);
        Self(value)
    }

    /// `self` の `x` 乗を計算する
    pub fn pow(&self, mut x: u32) -> Self {
        let mut a = *self;
        let mut r = Self::from_raw(1);

        while x > 0 {
            if x & 1 == 1 {
                r *= a;
            }

            a *= a;
            x >>= 1;
        }

        r
    }

    /// `self` の乗法逆元を計算する  
    /// フェルマーの小定理より、`self` の $`P - 2`$ 乗を計算している (`P` が素数であることを前提としている)
    pub fn inv(&self) -> Self {
        self.pow(P - 2)
    }
}

impl<const P: u32> Add for ModInt<P> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self((self.0 + rhs.0) % P)
    }
}

impl<const P: u32> Sub for ModInt<P> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self((P + self.0 - rhs.0) % P)
    }
}

impl<const P: u32> Mul for ModInt<P> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(((self.0 as u64 * rhs.0 as u64) % P as u64) as u32)
    }
}

impl<const P: u32> Div for ModInt<P> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv()
    }
}

impl<const P: u32> AddAssign for ModInt<P> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.0 %= P;
    }
}

impl<const P: u32> SubAssign for ModInt<P> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 += P - rhs.0;
        self.0 %= P;
    }
}

impl<const P: u32> MulAssign for ModInt<P> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}

impl<const P: u32> DivAssign for ModInt<P> {
    fn div_assign(&mut self, rhs: Self) {
        *self *= rhs.inv()
    }
}

impl<const P: u32> Neg for ModInt<P> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self((P - self.0) % P)
    }
}

impl<const P: u32> Display for ModInt<P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}

macro_rules! impl_op_for_modint {
    ($($t: ty), *) => {
        $(
            impl<const P: u32> From<$t> for ModInt<P> {
                fn from(value: $t) -> Self {
                    Self((P as $t + value % P as $t) as u32 % P)
                }
            }

            impl<const P: u32> Add<$t> for ModInt<P> {
                type Output = Self;
                fn add(self, rhs: $t) -> Self::Output {
                    self + Self::from(rhs)
                }
            }

            impl<const P: u32> Add<ModInt<P>> for $t {
                type Output = ModInt<P>;
                fn add(self, rhs: ModInt<P>) -> Self::Output {
                    Self::Output::from(self) + rhs
                }
            }

            impl<const P: u32> Sub<$t> for ModInt<P> {
                type Output = Self;
                fn sub(self, rhs: $t) -> Self::Output {
                    self - Self::from(rhs)
                }
            }

            impl<const P: u32> Sub<ModInt<P>> for $t {
                type Output = ModInt<P>;
                fn sub(self, rhs: ModInt<P>) -> Self::Output {
                    Self::Output::from(self) - rhs
                }
            }

            impl<const P: u32> Mul<$t> for ModInt<P> {
                type Output = Self;
                fn mul(self, rhs: $t) -> Self::Output {
                    self * Self::from(rhs)
                }
            }

            impl<const P: u32> Mul<ModInt<P>> for $t {
                type Output = ModInt<P>;
                fn mul(self, rhs: ModInt<P>) -> Self::Output {
                    Self::Output::from(self) * rhs
                }
            }

            impl<const P: u32> Div<$t> for ModInt<P> {
                type Output = Self;
                fn div(self, rhs: $t) -> Self::Output {
                    self / Self::from(rhs)
                }
            }

            impl<const P: u32> Div<ModInt<P>> for $t {
                type Output = ModInt<P>;
                fn div(self, rhs: ModInt<P>) -> Self::Output {
                    Self::Output::from(self) / rhs
                }
            }

            impl<const P: u32> AddAssign<$t> for ModInt<P> {
                fn add_assign(&mut self, rhs: $t) {
                    *self += Self::from(rhs)
                }
            }

            impl<const P: u32> SubAssign<$t> for ModInt<P> {
                fn sub_assign(&mut self, rhs: $t) {
                    *self -= Self::from(rhs)
                }
            }

            impl<const P: u32> MulAssign<$t> for ModInt<P> {
                fn mul_assign(&mut self, rhs: $t) {
                    *self *= Self::from(rhs)
                }
            }

            impl<const P: u32> DivAssign<$t> for ModInt<P> {
                fn div_assign(&mut self, rhs: $t) {
                    *self /= Self::from(rhs)
                }
            }
        )*
    };
}

impl_op_for_modint!(usize, isize, u64, i64, u32, i32);
