// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::rc::Rc;

fn f2<T1: ByteRepr>(a0: Ptr<Vec<Value<Vec<T1>>>>, init: Vec<T1>) {
    let __init = init;
    a0.with_mut(|__v: &mut Vec<Value<Vec<T1>>>| __v.push(Rc::new(RefCell::new(__init))))
}
