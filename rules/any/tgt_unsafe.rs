// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t1() -> Option<Box<dyn std::any::Any>> {
    None
}

unsafe fn f1() -> Option<Box<dyn std::any::Any>> {
    None
}

unsafe fn f6(a0: &mut Option<Box<dyn std::any::Any>>) {
    *a0 = None
}

unsafe fn f8(a0: Option<Box<dyn std::any::Any>>) -> bool {
    a0.is_some()
}

unsafe fn f14<T1: 'static>(a0: &mut Option<Box<dyn std::any::Any>>, init: cpp_type!("std::decay_t<T1>", T1)) -> &mut T1 {
    *a0 = Some(Box::new(init));
    a0.as_mut().unwrap().downcast_mut::<T1>().unwrap()
}

unsafe fn f19<T1: 'static>(a0: *mut Option<Box<dyn std::any::Any>>) -> *mut T1 {
    match (*a0).as_mut().and_then(|__b| __b.downcast_mut::<T1>()) {
        Some(__v) => __v as *mut T1,
        None => std::ptr::null_mut(),
    }
}

unsafe fn f27<T1: 'static>(init: cpp_type!("std::decay_t<T1>", T1)) -> Option<Box<dyn std::any::Any>> {
    Some(Box::new(init))
}
