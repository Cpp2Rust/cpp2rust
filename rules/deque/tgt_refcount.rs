// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::rc::Rc;

fn t4<T1>() -> Ptr<T1> {
    Ptr::null()
}

fn t5<T1>() -> Ptr<T1> {
    Ptr::null()
}

fn f1<T1>(a0: &mut Vec<T1>) -> &mut T1 {
    let __n = a0.len() - 1;
    &mut a0[__n]
}

fn f2<T1>(a0: &mut Vec<T1>) -> &mut T1 {
    &mut a0[0]
}

fn f10<T1>(a0: &mut Vec<T1>, a1: Vec<T1>) -> &mut Vec<T1> {
    *a0 = a1;
    a0
}

fn f11<'a, T1>(a0: &'a mut Vec<T1>, a1: &mut Vec<T1>) -> &'a mut Vec<T1> {
    *a0 = std::mem::take(&mut *a1);
    a0
}
