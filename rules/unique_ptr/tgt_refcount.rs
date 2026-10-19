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

fn f5<T1: ByteRepr>(a0: Ptr<Option<Value<T1>>>, a1: Ptr<T1>) {
    let _p: Ptr<_> = a1;
    a0.write(_p.to_owned_opt())
}

fn f6<T1: ByteRepr>(a0: Ptr<Option<Value<Box<[T1]>>>>, a1: Ptr<T1>) {
    a0.write(a1.to_owned_opt())
}

fn f7<T1>(a0: Value<T1>) -> Ptr<T1> {
    a0.as_pointer()
}

fn f8<T1>(init: T1) -> Option<Value<T1>> {
    Some(Rc::new(RefCell::new(init)))
}

fn f9<T1: ByteRepr>(a0: Ptr<Option<Value<Box<[T1]>>>>) {
    a0.write(None)
}

fn f16<T1: ByteRepr>(a0: Ptr<Option<Value<T1>>>) {
    a0.write(None)
}

fn f10<T1>() -> Option<Value<T1>> {
    None
}

fn f11<T1>() -> Option<Value<Box<[T1]>>> {
    None
}

fn f12<T1>(a0: &mut Option<Value<T1>>) -> Option<Value<T1>> {
    a0.take()
}

fn f13<T1>(a0: &mut Option<Value<Box<[T1]>>>) -> Option<Value<Box<[T1]>>> {
    a0.take()
}

fn f14<T1: ByteRepr>(a0: Ptr<Option<Value<T1>>>, a1: &mut Option<Value<T1>>) {
    a0.write(a1.take())
}

fn f15<T1: ByteRepr>(a0: Ptr<Option<Value<Box<[T1]>>>>, a1: &mut Option<Value<Box<[T1]>>>) {
    a0.write(a1.take())
}

fn f17<T1>(a0: Value<T1>) -> Ptr<T1> {
    a0.as_pointer()
}

fn f18<T1>(a0: Value<T1>) -> Ptr<T1> {
    a0.as_pointer()
}

fn f19<T1>(a0: Value<T1>, a1: usize) -> Ptr<T1> {
    (a0.as_pointer() as Ptr<T1>).offset(a1 as isize)
}

fn f20<T1>(a0: Option<Value<T1>>) -> bool {
    a0.is_none()
}

fn f21<T1>(a0: Option<Value<T1>>) -> bool {
    a0.is_some()
}
