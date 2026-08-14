// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

fn t1() -> u8 {
    Default::default()
}

fn f1<T1>(a0: &mut u8, a1: T1) -> u8
where
    u8: std::ops::Shl<T1, Output = u8>,
{
    *a0 << a1
}

fn f2<T1>(a0: &mut u8, a1: T1) -> u8
where
    u8: std::ops::Shr<T1, Output = u8>,
{
    *a0 >> a1
}

fn f3<T1>(a0: &mut u8, a1: T1) -> u8
where
    u8: std::ops::Shl<T1, Output = u8>,
{
    let n_ = *a0 << a1;
    *a0 = n_;
    *a0
}

fn f4<T1>(a0: &mut u8, a1: T1) -> u8
where
    u8: std::ops::Shr<T1, Output = u8>,
{
    let n_ = *a0 >> a1;
    *a0 = n_;
    *a0
}
