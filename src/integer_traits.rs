pub trait HasMaxValue {
    const MAX: Self;
}

pub trait HasMinValue {
    const MIN: Self;
}

macro_rules! impl_to_integers {
    ($($t: ty), *) => {
        $(
            impl HasMaxValue for $t {
                const MAX: $t = <$t>::MAX;
            }

            impl HasMinValue for $t {
                const MIN: $t = <$t>::MIN;
            }
        )*
    };
}

impl_to_integers!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
