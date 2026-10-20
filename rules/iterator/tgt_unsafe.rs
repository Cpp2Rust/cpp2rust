// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

fn t1() -> isize {
    0_isize
}

fn t2<T1>() -> T1 {
    unimplemented!()
}

fn t3<T1>() -> *mut T1 {
    ::std::ptr::null_mut()
}
