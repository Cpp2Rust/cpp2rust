// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t1<T1: Default + Clone, const T2: usize>() -> Vec<T1> {
    std::array::from_fn::<_, T2, _>(|_| <T1>::default()).to_vec()
}

fn t2() -> usize {
    0_usize
}

fn t3<T1>() -> T1 {
    unimplemented!()
}

fn t4<T1>() -> *mut T1 {
    ::std::ptr::null_mut()
}

fn t5<T1>() -> *const T1 {
    ::std::ptr::null()
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

unsafe fn f6<T1: Clone>(a0: Vec<T1>) -> Vec<T1> {
    a0.clone()
}

unsafe fn f7<T1: Clone>(a0: &mut Vec<T1>, a1: Vec<T1>) {
    *a0 = a1.clone()
}

unsafe fn f8<T1, const T2: usize>(a0: [T1; T2]) -> Vec<T1> {
    Vec::from(a0)
}
