// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

fn t1<T1: Default, T2: Default>() -> (T1, T2) {
    <(T1, T2)>::default()
}

unsafe fn f2<T1: Clone, T2: Clone>(a0: (T1, T2)) -> (T1, T2) {
    a0.clone()
}
unsafe fn f7<T1, T2>(a0: T1, a1: T2) -> (T1, T2) {
    (a0.into(), a1.into())
}
unsafe fn f9<T1, T2>(a0: T1, a1: T2) -> (T1, T2) {
    (a0.into(), a1.into())
}
unsafe fn f10<T1, T2>(a0: T1, a1: T2) -> (T1, T2) {
    (a0.into(), a1.into())
}

unsafe fn f12<T1: Default, T2: Default>(a0: &mut (T1, T2)) -> (T1, T2) {
    std::mem::take(&mut *a0)
}

unsafe fn f13<T1: Clone, T2: Clone>(a0: &mut (T1, T2), a1: (T1, T2)) {
    *a0 = a1.clone()
}

unsafe fn f14<T1: Default, T2: Default>(a0: &mut (T1, T2), a1: &mut (T1, T2)) {
    *a0 = std::mem::take(&mut *a1)
}

unsafe fn f15<T1, T2>(a0: (T1, T2)) -> T2 {
    a0.1
}

unsafe fn f16<T1, T2>(a0: (T1, T2)) -> T1 {
    a0.0
}

unsafe fn f17<T1, T2>(a0: &mut (T1, T2)) -> *mut T1 {
    &mut a0.0
}

unsafe fn f18<T1, T2>(a0: &mut (T1, T2)) -> *mut T2 {
    &mut a0.1
}

unsafe fn f19<T1, T2>(a0: &(T1, T2)) -> *const T1 {
    &a0.0
}

unsafe fn f20<T1, T2>(a0: &(T1, T2)) -> *const T2 {
    &a0.1
}
