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

fn f1<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0
}

fn f3<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0
}

fn f5<T1: ByteRepr + Default, const T2: usize>(a0: Ptr<[T1; T2]>, a1: &mut [T1; T2]) {
    a0.write(std::array::from_fn(|__i| std::mem::take(&mut a1[__i])))
}

fn f7<T1: Clone + ByteRepr, const T2: usize>(a0: Ptr<[T1; T2]>, a1: [T1; T2]) {
    a0.write(a1)
}

fn f9<const T1: usize, T2>(a0: Ptr<T2>) -> Ptr<T2> {
    a0.offset(T1 as isize)
}

fn f10<const T1: usize, T2>(a0: Ptr<T2>) -> Ptr<T2> {
    a0.offset(T1 as isize)
}

fn f11<const T1: usize, T2>(a0: Ptr<T2>) -> Ptr<T2> {
    a0.offset(T1 as isize)
}

fn f12<const T1: usize, T2>(a0: Ptr<T2>) -> Ptr<T2> {
    a0.offset(T1 as isize)
}
