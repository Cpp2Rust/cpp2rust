// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::rc::Rc;

fn f1<T1: 'static>(a0: Ptr<Vec<T1>>) -> Ptr<T1> {
    a0.decay().to_last()
}

fn f2<T1: 'static>(a0: Ptr<Vec<T1>>) -> Ptr<T1> {
    a0.decay()
}

fn f10<T1: Clone + ByteRepr>(a0: Ptr<Vec<T1>>, a1: Vec<T1>) {
    a0.write(a1)
}

fn f11<T1: ByteRepr + Clone>(a0: Ptr<Vec<T1>>, a1: &mut Vec<T1>) {
    a0.write(std::mem::take(&mut *a1))
}
