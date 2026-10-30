// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::rc::Rc;

fn t1<T1: Default, T2: Default>() -> (Value<T1>, Value<T2>) {
    (
        Rc::new(RefCell::new(T1::default())),
        Rc::new(RefCell::new(T2::default())),
    )
}

fn f2<T1: Clone, T2: Clone>(a0: (Value<T1>, Value<T2>)) -> (Value<T1>, Value<T2>) {
    (
        Rc::new(RefCell::new(a0.0.borrow().clone())),
        Rc::new(RefCell::new(a0.1.borrow().clone())),
    )
}

fn f7<T1, T2>(a0: T1, a1: T2) -> (Value<T1>, Value<T2>) {
    (Rc::new(RefCell::new(a0)), Rc::new(RefCell::new(a1)))
}

fn f9<T1, T2>(a0: T1, a1: T2) -> (Value<T1>, Value<T2>) {
    (Rc::new(RefCell::new(a0)), Rc::new(RefCell::new(a1)))
}

fn f10<T1, T2>(a0: T1, a1: T2) -> (Value<T1>, Value<T2>) {
    (Rc::new(RefCell::new(a0)), Rc::new(RefCell::new(a1)))
}

fn f12<T1: Default, T2: Default>(a0: &mut (Value<T1>, Value<T2>)) -> (Value<T1>, Value<T2>) {
    std::mem::take(&mut *a0)
}

fn f13<T1: Clone + ByteRepr, T2: Clone + ByteRepr>(
    a0: Ptr<(Value<T1>, Value<T2>)>,
    a1: (Value<T1>, Value<T2>),
) {
    a0.write((
        Rc::new(RefCell::new(a1.0.borrow().clone())),
        Rc::new(RefCell::new(a1.1.borrow().clone())),
    ))
}

fn f14<T1: Default + ByteRepr, T2: Default + ByteRepr>(
    a0: Ptr<(Value<T1>, Value<T2>)>,
    a1: &mut (Value<T1>, Value<T2>),
) {
    a0.write(std::mem::take(&mut *a1))
}

fn f15<T1, T2>(a0: (Value<T1>, Value<T2>)) -> Value<T2> {
    a0.1
}

fn f16<T1, T2>(a0: (Value<T1>, Value<T2>)) -> Value<T1> {
    a0.0
}

fn f17<T1, T2>(a0: (Value<T1>, Value<T2>)) -> Ptr<T1> {
    a0.0.as_pointer()
}

fn f18<T1, T2>(a0: (Value<T1>, Value<T2>)) -> Ptr<T2> {
    a0.1.as_pointer()
}

fn f19<T1, T2>(a0: (Value<T1>, Value<T2>)) -> Ptr<T1> {
    a0.0.as_pointer()
}

fn f20<T1, T2>(a0: (Value<T1>, Value<T2>)) -> Ptr<T2> {
    a0.1.as_pointer()
}

fn f21<T1, T2>(a0: (Value<T1>, Value<T2>)) -> Ptr<T1> {
    a0.0.as_pointer()
}

fn f22<T1, T2>(a0: (Value<T1>, Value<T2>)) -> Ptr<T2> {
    a0.1.as_pointer()
}

fn f23<T1: TryFrom<T3>, T2: TryFrom<T4>, T3: Clone, T4: Clone>(
    a0: &mut (Value<T3>, Value<T4>),
) -> (Value<T1>, Value<T2>) {
    (
        Rc::new(RefCell::new(a0.0.borrow().clone().try_into().ok().expect("failed conversion"))),
        Rc::new(RefCell::new(a0.1.borrow().clone().try_into().ok().expect("failed conversion"))),
    )
}

fn f24<T1: TryFrom<T3>, T2: TryFrom<T4>, T3: Clone, T4: Clone>(
    a0: (Value<T3>, Value<T4>),
) -> (Value<T1>, Value<T2>) {
    (
        Rc::new(RefCell::new(a0.0.borrow().clone().try_into().ok().expect("failed conversion"))),
        Rc::new(RefCell::new(a0.1.borrow().clone().try_into().ok().expect("failed conversion"))),
    )
}
