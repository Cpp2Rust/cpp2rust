// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::rc::Rc;

fn t1() -> Option<Rc<dyn std::any::Any>> {
    None
}

fn f1() -> Option<Rc<dyn std::any::Any>> {
    None
}

fn f6(a0: &mut Option<Rc<dyn std::any::Any>>) {
    *a0 = None
}

fn f8(a0: Option<Rc<dyn std::any::Any>>) -> bool {
    a0.is_some()
}

fn f14<T1: 'static>(a0: &mut Option<Rc<dyn std::any::Any>>, init: cpp_type!("std::decay_t<T1>", T1)) -> Ptr<T1> {
    let __value: Value<T1> = Rc::new(RefCell::new(init));
    let __ptr = __value.as_pointer();
    *a0 = Some(__value);
    __ptr
}

fn f19<T1: 'static>(a0: Ptr<Option<Rc<dyn std::any::Any>>>) -> Ptr<T1> {
    a0.with(|__a: &Option<Rc<dyn std::any::Any>>| match __a.clone().map(|__r| __r.downcast::<RefCell<T1>>()) {
        Some(Ok(__v)) => __v.as_pointer(),
        _ => Ptr::null(),
    })
}

fn f27<T1: 'static>(init: cpp_type!("std::decay_t<T1>", T1)) -> Option<Rc<dyn std::any::Any>> {
    Some(Rc::new(RefCell::new(init)))
}
