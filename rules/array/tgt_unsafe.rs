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

fn f1<T1, const T2: usize>(a0: [T1; T2]) -> *const T1 {
    a0.as_ptr()
}

fn f2<T1, const T2: usize>(a0: [T1; T2]) -> usize {
    T2
}

fn f3<T1, const T2: usize>(a0: &mut [T1; T2]) -> *mut T1 {
    a0.as_mut_ptr()
}

unsafe fn f4<T1: MoveCtorUnsafe, const T2: usize>(a0: &mut [T1; T2]) -> [T1; T2] {
    std::array::from_fn(|__i| T1::move_ctor(&mut a0[__i]))
}

unsafe fn f5<'a, T1: MoveAssignUnsafe, const T2: usize>(
    a0: &'a mut [T1; T2],
    a1: &mut [T1; T2],
) -> &'a mut [T1; T2] {
    for __i in 0..T2 {
        T1::move_assign(&mut a0[__i], &mut a1[__i]);
    }
    a0
}

fn f6<T1: Clone, const T2: usize>(a0: [T1; T2]) -> [T1; T2] {
    a0.clone()
}

unsafe fn f7<'a, T1: CopyAssignUnsafe, const T2: usize>(
    a0: &'a mut [T1; T2],
    a1: &[T1; T2],
) -> &'a mut [T1; T2] {
    for __i in 0..T2 {
        T1::copy_assign(&mut a0[__i], &a1[__i]);
    }
    a0
}

fn f8<T1, const T2: usize>(a0: [T1; T2]) -> [T1; T2] {
    a0
}

fn f9<const T1: usize, T2, const T3: usize>(a0: &mut [T2; T3]) -> &mut T2 {
    &mut a0[T1]
}

fn f10<const T1: usize, T2, const T3: usize>(a0: &[T2; T3]) -> &T2 {
    &a0[T1]
}

fn f11<const T1: usize, T2, const T3: usize>(a0: &mut [T2; T3]) -> &mut T2 {
    &mut a0[T1]
}

fn f12<const T1: usize, T2, const T3: usize>(a0: &[T2; T3]) -> &T2 {
    &a0[T1]
}
