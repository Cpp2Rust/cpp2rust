// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

fn t1() -> u8 {
    Default::default()
}

fn t2() -> usize {
    0_usize
}

fn t3() -> isize {
    0_isize
}

fn t4() -> isize {
    0_isize
}

fn t5() -> usize {
    0_usize
}

fn t6() -> isize {
    0_isize
}

fn t9() -> usize {
    0_usize
}

fn f1(a0: &mut u8, a1: u32) -> u8 {
    ((*a0 as u32) << a1) as u8
}

fn f2(a0: &mut u8, a1: u32) -> u8 {
    ((*a0 as u32) >> a1) as u8
}

fn f3(a0: &mut u8, a1: u32) -> u8 {
    let __n = ((*a0 as u32) << a1) as u8;
    *a0 = __n;
    *a0
}

fn f4(a0: &mut u8, a1: u32) -> u8 {
    let __n = ((*a0 as u32) >> a1) as u8;
    *a0 = __n;
    *a0
}

fn f5<T1>(a0: u8, a1: T1) -> u8
where
    u32: ::std::ops::Shl<T1, Output = u32>,
{
    ((a0 as u32) << a1) as u8
}
