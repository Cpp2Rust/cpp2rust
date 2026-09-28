// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

fn t1() -> u8 {
    Default::default()
}

fn t2() -> usize {
    0_usize
}

fn t3() -> *mut usize {
    std::ptr::null_mut()
}

fn t4() -> *const usize {
    std::ptr::null()
}

fn t5() -> usize {
    0_usize
}

fn t6() -> isize {
    0_isize
}

fn t7() -> *mut isize {
    std::ptr::null_mut()
}

fn t8() -> *const isize {
    std::ptr::null()
}

fn f1(a0: &mut u8, a1: u32) -> u8 {
    *a0 << a1
}

fn f2(a0: &mut u8, a1: u32) -> u8 {
    *a0 >> a1
}

fn f3(a0: &mut u8, a1: u32) -> u8 {
    let n_ = *a0 << a1;
    *a0 = n_;
    *a0
}

fn f4(a0: &mut u8, a1: u32) -> u8 {
    let n_ = *a0 >> a1;
    *a0 = n_;
    *a0
}
