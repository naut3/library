/// 半群
pub trait SemiGroup {
    type S;
    fn op(lhs: &Self::S, rhs: &Self::S) -> Self::S;
}

/// 帯(冪等半群)
pub trait Band {
    type S;
    fn op(lhs: &Self::S, rhs: &Self::S) -> Self::S;
}

/// モノイド
pub trait Monoid {
    type S;
    fn op(lhs: &Self::S, rhs: &Self::S) -> Self::S;
    const E: Self::S;
}

pub struct Min<T> {
    _marker: std::marker::PhantomData<T>,
}

pub struct Max<T> {
    _marker: std::marker::PhantomData<T>,
}

pub struct Add<T> {
    _marker: std::marker::PhantomData<T>,
}

pub struct Mul<T> {
    _marker: std::marker::PhantomData<T>,
}

pub struct BitAnd<T> {
    _marker: std::marker::PhantomData<T>,
}

pub struct BitOr<T> {
    _marker: std::marker::PhantomData<T>,
}

pub struct BitXor<T> {
    _marker: std::marker::PhantomData<T>,
}

macro_rules! impl_to_integers {
    ($($t: ty), *) => {
        $(
            impl SemiGroup for Min<$t> {
                type S = $t;
                fn op(lhs: &$t, rhs: &$t) -> $t {
                    std::cmp::min(*lhs, *rhs)
                }
            }

            impl SemiGroup for Max<$t> {
                type S = $t;
                fn op(lhs: &$t, rhs: &$t) -> $t {
                    std::cmp::max(*lhs, *rhs)
                }
            }

            impl SemiGroup for Add<$t> {
                type S = $t;
                fn op(lhs: &$t, rhs: &$t) -> $t {
                    lhs + rhs
                }
            }

            impl SemiGroup for Mul<$t> {
                type S = $t;
                fn op(lhs: &$t, rhs: &$t) -> $t {
                    lhs * rhs
                }
            }

            impl SemiGroup for BitAnd<$t> {
                type S = $t;
                fn op(lhs: &$t, rhs: &$t) -> $t {
                    lhs & rhs
                }
            }

            impl SemiGroup for BitOr<$t> {
                type S = $t;
                fn op(lhs: &$t, rhs: &$t) -> $t {
                    lhs | rhs
                }
            }

            impl SemiGroup for BitXor<$t> {
                type S = $t;
                fn op(lhs: &$t, rhs: &$t) -> $t {
                    lhs ^ rhs
                }
            }

            impl Band for Min<$t> {
                type S = $t;
                fn op(lhs: &$t, rhs: &$t) -> $t {
                    std::cmp::min(*lhs, *rhs)
                }
            }

            impl Band for Max<$t> {
                type S = $t;
                fn op(lhs: &$t, rhs: &$t) -> $t {
                    std::cmp::max(*lhs, *rhs)
                }
            }

            impl Band for BitAnd<$t> {
                type S = $t;
                fn op(lhs: &$t, rhs: &$t) -> $t {
                    lhs & rhs
                }
            }

            impl Band for BitOr<$t> {
                type S = $t;
                fn op(lhs: &$t, rhs: &$t) -> $t {
                    lhs | rhs
                }
            }

            impl Monoid for Min<$t> {
                type S = $t;
                fn op(lhs: &$t, rhs: &$t) -> $t {
                    std::cmp::min(*lhs, *rhs)
                }
                const E: $t = <$t>::MAX;
            }

            impl Monoid for Max<$t> {
                type S = $t;
                fn op(lhs: &$t, rhs: &$t) -> $t {
                    std::cmp::max(*lhs, *rhs)
                }
                const E: $t = <$t>::MIN;
            }

            impl Monoid for Add<$t> {
                type S = $t;
                fn op(lhs: &$t, rhs: &$t) -> $t {
                    lhs + rhs
                }
                const E: $t = 0;
            }

            impl Monoid for Mul<$t> {
                type S = $t;
                fn op(lhs: &$t, rhs: &$t) -> $t {
                    lhs * rhs
                }
                const E: $t = 1;
            }

            impl Monoid for BitAnd<$t> {
                type S = $t;
                fn op(lhs: &$t, rhs: &$t) -> $t {
                    lhs & rhs
                }
                const E: $t = <$t>::MAX;
            }

            impl Monoid for BitOr<$t> {
                type S = $t;
                fn op(lhs: &$t, rhs: &$t) -> $t {
                    lhs | rhs
                }
                const E: $t = 0;
            }

            impl Monoid for BitXor<$t> {
                type S = $t;
                fn op(lhs: &$t, rhs: &$t) -> $t {
                    lhs ^ rhs
                }
                const E: $t = 0;
            }
        )*
    };
}

// \[WARN\] 符号付き整数の bitwise な演算は単位元を間違えている気がする
impl_to_integers!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
