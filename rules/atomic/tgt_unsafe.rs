// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// TODO: use proper atomic translation

fn t1<T1: Default>() -> T1 {
    T1::default()
}

unsafe fn f1<T1: Default>() -> T1 {
    T1::default()
}

unsafe fn f2<T1>(a0: T1) -> T1 {
    a0
}
