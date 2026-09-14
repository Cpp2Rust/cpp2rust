// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t1<T1>() -> Vec<T1> {
    Default::default()
}

unsafe fn f1<T1>(a0: Vec<T1>) -> *const T1 {
    a0.as_ptr()
}

unsafe fn f2<T1>(a0: Vec<T1>) -> usize {
    a0.len()
}

unsafe fn f3<T1>(a0: &mut Vec<T1>) -> *mut T1 {
    a0.as_mut_ptr()
}

unsafe fn f4<T1>(a0: &mut Vec<T1>) -> Vec<T1> {
    std::mem::take(&mut *a0)
}

unsafe fn f5<T1>(a0: &mut Vec<T1>, a1: &mut Vec<T1>) {
    *a0 = std::mem::take(&mut *a1)
}
