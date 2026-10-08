extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn double_it_0(mut x: Ptr<i32>) {
    {
        x.with_mut(|__v| *__v = *__v * 2)
    };
}
pub fn maybe_call_1(mut cb: FnPtr<fn(Ptr<i32>)>, mut x: Ptr<i32>) {
    if !(cb).is_null() {
        ({ cb.call((x).clone()) });
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(5));
    ({ maybe_call_1(FnPtr::<fn(Ptr<i32>)>::new(double_it_0), (a.as_pointer())) });
    assert!(((*a.borrow()) == 10));
    let b: Value<i32> = Rc::new(RefCell::new(5));
    ({ maybe_call_1(FnPtr::<fn(Ptr<i32>)>::null(), (b.as_pointer())) });
    assert!(((*b.borrow()) == 5));
    let mut fn_: FnPtr<fn(Ptr<i32>)> = FnPtr::<fn(Ptr<i32>)>::null();
    if !(!(fn_).is_null()) {
        fn_ = FnPtr::<fn(Ptr<i32>)>::new(double_it_0);
    }
    let c: Value<i32> = Rc::new(RefCell::new(3));
    if !(fn_).is_null() {
        ({ fn_.call((c.as_pointer())) });
    }
    assert!(((*c.borrow()) == 6));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
