// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#[derive(Default)]
struct T1;

#[derive(Default)]
struct T2;

fn types() {
    let t1: (T1, T2) = (T1::default(), T2::default());
}

unsafe fn f1<T1, T2>(a0: (T1, T2)) -> T2 {
    a0.1
}
unsafe fn f2<T1: Clone, T2: Clone>(a0: (T1, T2)) -> (T1, T2) {
    a0.clone()
}
unsafe fn f4<T1, T2>(a0: T1, a1: T2) -> (T1, T2) {
    (a0.into(), a1.into())
}
unsafe fn f5<T1, T2>(a0: T1, a1: T2) -> (T1, T2) {
    (a0.into(), a1.into())
}
unsafe fn f6<T1, T2>(a0: T1, a1: T2) -> (T1, T2) {
    (a0.into(), a1.into())
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

unsafe fn f11<T1, T2>(a0: (T1, T2)) -> T1 {
    a0.0
}
