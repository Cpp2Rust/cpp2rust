// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::rc::Rc;

fn f1<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0.to_last()
}

fn f2<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0
}

fn f7<T1: ByteRepr>(a0: Ptr<Vec<Value<Vec<T1>>>>, a1: Value<Vec<T1>>) {
    a0.with_mut(|__v: &mut Vec<Value<Vec<T1>>>| __v.push(a1))
}

fn f10<T1: Clone + ByteRepr>(a0: Ptr<Vec<T1>>, a1: Vec<T1>) {
    a0.write(a1.clone())
}

fn f11<T1: ByteRepr + Clone>(a0: Ptr<Vec<T1>>, a1: &mut Vec<T1>) {
    a0.write(std::mem::take(&mut *a1))
}
