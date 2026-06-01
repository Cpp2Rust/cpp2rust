// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

// TODO: it should be VecDeque, but we don't have the neccessary infrastructure in Ptr<> to
// make this work.
fn t1<T1>() -> Vec<T1> {
    Default::default()
}

unsafe fn f1<T1>(a0: &mut Vec<T1>) -> *const T1 {
    (a0.last_mut().unwrap())
}

unsafe fn f2<T1>(a0: &mut Vec<T1>) -> *const T1 {
    ((a0).first_mut().unwrap())
}

unsafe fn f3<T1>(a0: Vec<T1>) -> bool {
    a0.is_empty()
}

unsafe fn f4<T1: Default>(a0: &mut Vec<T1>, a1: &mut T1) {
    a0.push(std::mem::take(&mut *a1))
}

unsafe fn f5<T1>(a0: &mut Vec<T1>) -> T1 {
    a0.remove(0)
}

unsafe fn f7<T1>(a0: &mut Vec<Vec<T1>>, a1: Vec<T1>) {
    a0.push(a1)
}
