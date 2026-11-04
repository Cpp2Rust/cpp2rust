// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t1<T1: Default, const T2: usize>() -> [T1; T2] {
    std::array::from_fn::<_, T2, _>(|_| <T1>::default())
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

fn t6<T2>() -> T2 {
    unimplemented!()
}

unsafe fn f1<T1, const T2: usize>(a0: [T1; T2]) -> *const T1 {
    a0.as_ptr()
}

unsafe fn f2<T1, const T2: usize>(a0: [T1; T2]) -> usize {
    T2
}

unsafe fn f3<T1, const T2: usize>(a0: &mut [T1; T2]) -> *mut T1 {
    a0.as_mut_ptr()
}

unsafe fn f4<T1: Default, const T2: usize>(a0: &mut [T1; T2]) -> [T1; T2] {
    std::array::from_fn(|__i| std::mem::take(&mut a0[__i]))
}

unsafe fn f5<T1: Default, const T2: usize>(a0: &mut [T1; T2], a1: &mut [T1; T2]) {
    *a0 = std::array::from_fn(|__i| std::mem::take(&mut a1[__i]))
}

unsafe fn f6<T1: Clone, const T2: usize>(a0: [T1; T2]) -> [T1; T2] {
    a0.clone()
}

unsafe fn f7<T1: Clone, const T2: usize>(a0: &mut [T1; T2], a1: [T1; T2]) {
    *a0 = a1.clone()
}

unsafe fn f8<T1, const T2: usize>(a0: [T1; T2]) -> [T1; T2] {
    a0
}

unsafe fn f9<const T1: usize, T2, const T3: usize>(a0: &mut [T2; T3]) -> *mut T2 {
    a0.as_mut_ptr().offset(T1 as isize)
}

unsafe fn f10<const T1: usize, T2, const T3: usize>(a0: &[T2; T3]) -> *const T2 {
    a0.as_ptr().offset(T1 as isize)
}

unsafe fn f11<const T1: usize, T2, const T3: usize>(a0: &mut [T2; T3]) -> *mut T2 {
    a0.as_mut_ptr().offset(T1 as isize)
}

unsafe fn f12<const T1: usize, T2, const T3: usize>(a0: &[T2; T3]) -> *const T2 {
    a0.as_ptr().offset(T1 as isize)
}
