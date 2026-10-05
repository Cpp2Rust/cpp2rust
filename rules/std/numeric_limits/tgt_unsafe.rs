// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

pub trait HasMinMax {
    const MIN: Self;
    const MAX: Self;
}

macro_rules! impl_hasminmax {
    ($($t:ty),* $(,)?) => {
        $(
            impl HasMinMax for $t {
                const MIN: Self = <$t>::MIN;
                const MAX: Self = <$t>::MAX;
            }
        )*
    };
}

impl_hasminmax!(
    u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize
);

unsafe fn f1<T1: HasMinMax>() -> T1 {
    <T1>::MAX
}

unsafe fn f2<T1: HasMinMax>() -> T1 {
    <T1>::MIN
}
