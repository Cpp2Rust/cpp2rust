// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::rc::Rc;

fn t1<T1>() -> Option<Value<T1>> {
    None
}

fn t2<T1>() -> Option<Value<Box<[T1]>>> {
    None
}

fn f1<T1: Default>(a0: usize) -> Option<Value<Box<[T1]>>> {
    Some(Rc::new(RefCell::new(
        (0..a0).map(|_| <T1>::default()).collect::<Box<[_]>>(),
    )))
}

fn f2<T1>(a0: Value<T1>) -> Ptr<T1> {
    a0.as_pointer()
}

fn f3<T1>(a0: Ptr<T1>) -> Option<Value<T1>> {
    a0.to_owned_opt()
}

fn f4<T1>(a0: Ptr<T1>) -> Option<Value<T1>> {
    a0.to_owned_opt()
}

fn f5<T1>(a0: &mut Option<Value<T1>>, a1: Ptr<T1>) {
    let _p: Ptr<_> = a1;
    *a0 = _p.to_owned_opt()
}

fn f6<T1>(a0: &mut Option<Value<Box<[T1]>>>, a1: Ptr<T1>) {
    *a0 = a1.to_owned_opt()
}

fn f7<T1>(a0: Value<T1>) -> Ptr<T1> {
    a0.as_pointer()
}

fn f8<T1>(a0: T1) -> Option<Value<T1>> {
    Some(Rc::new(RefCell::new(a0)))
}

fn f9<T1>(a0: &mut Option<Value<Box<[T1]>>>) {
    *a0 = None
}

fn f10<T1>() -> Option<Value<T1>> {
    None
}

fn f11<T1>() -> Option<Value<Box<[T1]>>> {
    None
}
