// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t1<T1>() -> Vec<T1> {
    Default::default()
}

unsafe fn f1<T1>(a0: Vec<T1>) -> *const T1 {
    a0.as_ptr()
}

// TODO: this should return usize. However std::size_t is translated as unsigned long which in turn
// gets translated to u64.
unsafe fn f2<T1>(a0: Vec<T1>) -> u64 {
    a0.len() as u64
}

unsafe fn f3<T1>(a0: &mut Vec<T1>) -> *mut T1 {
    a0.as_mut_ptr()
}
