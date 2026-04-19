// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
struct T1;

#[derive(Default)]
struct T2;

fn types() {
    let t1: (Value<T1>, Value<T2>) = (
        Rc::new(RefCell::new(T1::default())),
        Rc::new(RefCell::new(T2::default())),
    );
}

fn f1<T1, T2>(a0: (Value<T1>, Value<T2>)) -> Value<T2> {
    a0.1
}

fn f2<T1: Clone, T2: Clone>(a0: (Value<T1>, Value<T2>)) -> (Value<T1>, Value<T2>) {
    (
        Rc::new(RefCell::new(a0.0.borrow().clone())),
        Rc::new(RefCell::new(a0.1.borrow().clone())),
    )
}

fn f4<T1, T2>(a0: T1, a1: T2) -> (Value<T1>, Value<T2>) {
    (
        Rc::new(RefCell::new(a0.try_into().expect("failed conversion"))),
        Rc::new(RefCell::new(a1.try_into().expect("failed conversion"))),
    )
}

fn f5<T1, T2>(a0: T1, a1: T2) -> (Value<T1>, Value<T2>) {
    (
        Rc::new(RefCell::new(a0.try_into().expect("failed conversion"))),
        Rc::new(RefCell::new(a1.try_into().expect("failed conversion"))),
    )
}

fn f6<T1, T2>(a0: T1, a1: T2) -> (Value<T1>, Value<T2>) {
    (
        Rc::new(RefCell::new(a0.try_into().expect("failed conversion"))),
        Rc::new(RefCell::new(a1.try_into().expect("failed conversion"))),
    )
}

fn f7<T1, T2>(a0: T1, a1: T2) -> (Value<T1>, Value<T2>) {
    (
        Rc::new(RefCell::new(a0.try_into().expect("failed conversion"))),
        Rc::new(RefCell::new(a1.try_into().expect("failed conversion"))),
    )
}

fn f9<T1, T2>(a0: T1, a1: T2) -> (Value<T1>, Value<T2>) {
    (
        Rc::new(RefCell::new(a0.try_into().expect("failed conversion"))),
        Rc::new(RefCell::new(a1.try_into().expect("failed conversion"))),
    )
}

fn f10<T1, T2>(a0: T1, a1: T2) -> (Value<T1>, Value<T2>) {
    (
        Rc::new(RefCell::new(a0.try_into().expect("failed conversion"))),
        Rc::new(RefCell::new(a1.try_into().expect("failed conversion"))),
    )
}

fn f11<T1, T2>(a0: (Value<T1>, Value<T2>)) -> Value<T1> {
    a0.0
}
