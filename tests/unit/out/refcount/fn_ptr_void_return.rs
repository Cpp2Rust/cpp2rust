extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn negate_0(mut x: Ptr<i32>) {
    x.write({ -(x.read()) });
}
pub fn zero_out_1(mut x: Ptr<i32>) {
    x.write(0);
}
pub fn run_2(mut fn_: FnPtr<fn(Ptr<i32>)>, mut x: Ptr<i32>) {
    ({ fn_.call((x).clone()) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(42));
    ({ run_2(FnPtr::<fn(Ptr<i32>)>::new(negate_0), (a.as_pointer())) });
    assert!(((*a.borrow()) == -42_i32));
    ({ run_2(FnPtr::<fn(Ptr<i32>)>::new(zero_out_1), (a.as_pointer())) });
    assert!(((*a.borrow()) == 0));
    let mut fn_: FnPtr<fn(Ptr<i32>)> = FnPtr::<fn(Ptr<i32>)>::new(negate_0);
    assert!(!((fn_).is_null()));
    let b: Value<i32> = Rc::new(RefCell::new(10));
    ({ fn_.call((b.as_pointer())) });
    assert!(((*b.borrow()) == -10_i32));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
