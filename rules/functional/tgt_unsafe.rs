// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

fn t1<T1>() -> *mut T1 {
    Default::default()
}

unsafe fn f1<T1>(a0: *mut T1) -> *mut T1 {
    a0
}

unsafe fn f2<T1>(a0: *mut T1) -> *mut T1 {
    a0
}

unsafe fn f3<T1>(a0: *mut T1) -> *mut T1 {
    a0
}
