/// [`ModInt`]は正の整数 $`P`$ を事前に設定してその剰余類 $`\mathbb{Z}/P\mathbb{Z}`$ を扱うことができる。
///  
/// ## Examples
///
/// `ModInt<P>` 同士の演算については、通常の数に対してのそれと同じような感覚で扱うことができる。
///
/// ```
/// use library::modint::ModInt;
///
/// type MInt = ModInt<998244353>;
///
/// let a = MInt::from_raw(10u32);
/// let b = MInt::from(998244353 - 5);
///
/// let c = a + b;
/// assert_eq!(c, MInt::from(5));
///
/// let d = a - b;
/// assert_eq!(d, MInt::from(15));
///
/// let e = a * b; // 10 * (-5)
/// assert_eq!(e, MInt::from(998244353 - 50));
///
/// let f = a / b; // 10 / (-5)
/// assert_eq!(f, MInt::from(998244353 - 2));
/// ```
///
/// `ModInt<P>`と通常の数を四則演算するときは、自動的に`ModInt<P>`に変換する。
///
/// ```
/// use library::modint::ModInt;
///
/// type MInt = ModInt<1_000_000_007>;
///
/// let mut a = MInt::default();
/// a = a + 6;
/// assert_eq!(a, MInt::from(6));
///
/// a /= 3;
/// assert_eq!(a, MInt::from(2));
///
/// a -= -5;
/// assert_eq!(a, MInt::from(7));
///
/// assert_eq!(MInt::from(1) / a, a.inv());
/// assert_eq!(a.pow(3), MInt::from(7 * 7 * 7));
/// ```
///
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Hash)]
pub struct ModInt<const P: u32>(u32);

impl<const P: u32> ModInt<P> {
    pub fn value(&self) -> u32 {
        return self.0;
    }

    pub fn new() -> Self {
        Self(0)
    }

    pub fn from_raw(value: u32) -> Self {
        Self(value)
    }

    pub fn inv(&self) -> Self {
        self.pow(P as u64 - 2)
    }

    pub fn pow(&self, mut x: u64) -> Self {
        let mut a = self.clone();
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
}

macro_rules! impl_to_integer_types {
    ($($t: ty), *) => {
        $(
            impl<const P: u32> From<$t> for ModInt<P> {
                fn from(value: $t) -> Self {
                    let v = ((value % (P as $t)) + P as $t) as u32;
                    Self(if v >= P { v - P } else { v })
                }
            }

            impl<const P: u32> std::ops::Add<$t> for ModInt<P> {
                type Output = Self;
                fn add(self, rhs: $t) -> Self::Output {
                    self + ModInt::<P>::from(rhs)
                }
            }

            impl<const P: u32> std::ops::AddAssign<$t> for ModInt<P> {
                fn add_assign(&mut self, rhs: $t) {
                    self.0 += ModInt::<P>::from(rhs).0;
                    if self.0 >= P {
                        self.0 -= P;
                    }
                }
            }

            impl<const P: u32> std::ops::Sub<$t> for ModInt<P> {
                type Output = Self;
                fn sub(self, rhs: $t) -> Self::Output {
                    self - ModInt::<P>::from(rhs)
                }
            }

            impl<const P: u32> std::ops::SubAssign<$t> for ModInt<P> {
                fn sub_assign(&mut self, rhs: $t) {
                    let m = ModInt::<P>::from(rhs);
                    if self.0 >= m.0 {
                        self.0 -= m.0;
                    } else {
                        self.0 += P - m.0;
                    }
                }
            }

            impl<const P: u32> std::ops::Mul<$t> for ModInt<P> {
                type Output = Self;
                fn mul(self, rhs: $t) -> Self::Output {
                    self * ModInt::<P>::from(rhs)
                }
            }

            impl<const P: u32> std::ops::MulAssign<$t> for ModInt<P> {
                fn mul_assign(&mut self, rhs: $t) {
                    let r = self.0 as u64;
                    self.0 = ((r * ModInt::<P>::from(rhs).0 as u64) % P as u64) as u32;
                }
            }

            impl<const P: u32> std::ops::Div<$t> for ModInt<P> {
                type Output = Self;
                fn div(self, rhs: $t) -> Self::Output {
                    self / ModInt::<P>::from(rhs)
                }
            }

            impl<const P: u32> std::ops::DivAssign<$t> for ModInt<P> {
                fn div_assign(&mut self, rhs: $t) {
                    *self *= ModInt::<P>::from(rhs).inv();
                }
            }
        )*
    };
}

impl_to_integer_types!(usize, isize, u64, i64, u32, i32);

impl<const P: u32> std::ops::Add for ModInt<P> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let r = self.0 + rhs.0;
        Self(if r >= P { r - P } else { r })
    }
}

impl<const P: u32> std::ops::AddAssign for ModInt<P> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        if self.0 >= P {
            self.0 -= P;
        }
    }
}

impl<const P: u32> std::ops::Sub for ModInt<P> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(if self.0 >= rhs.0 {
            self.0 - rhs.0
        } else {
            P + self.0 - rhs.0
        })
    }
}

impl<const P: u32> std::ops::SubAssign for ModInt<P> {
    fn sub_assign(&mut self, rhs: Self) {
        if self.0 >= rhs.0 {
            self.0 -= rhs.0;
        } else {
            self.0 += P - rhs.0;
        }
    }
}

impl<const P: u32> std::ops::Mul for ModInt<P> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(((self.0 as u64 * rhs.0 as u64) % P as u64) as u32)
    }
}

impl<const P: u32> std::ops::MulAssign for ModInt<P> {
    fn mul_assign(&mut self, rhs: Self) {
        let r = self.0 as u64;
        self.0 = ((r * rhs.0 as u64) % P as u64) as u32;
    }
}

impl<const P: u32> std::ops::Div for ModInt<P> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv()
    }
}

impl<const P: u32> std::ops::DivAssign for ModInt<P> {
    fn div_assign(&mut self, rhs: Self) {
        *self *= rhs.inv();
    }
}

impl<const P: u32> std::ops::Neg for ModInt<P> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new() - self
    }
}

impl<const P: u32> std::fmt::Display for ModInt<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
