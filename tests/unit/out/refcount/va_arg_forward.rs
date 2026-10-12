extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn inner_0(mut count: i32, ap: Ptr<VaArg>) -> i32 {
    let ap: Value<Ptr<VaArg>> = Rc::new(RefCell::new(ap));
    let mut total: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while (((i < count) as i32) != 0) {
        total += (*ap.borrow_mut()).arg::<i32>();
        i.postfix_inc();
    }
    return total;
}
pub fn outer_1(count: i32, __args: &[VaArg]) -> i32 {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    let count: Value<i32> = Rc::new(RefCell::new(count));
    let ap: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
    (*ap.borrow_mut()) = __args.as_pointer();
    let mut result: i32 = ({ inner_0((*count.borrow()), (*ap.borrow()).clone()) });
    return result;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((((({ outer_1(3, &[(10).into(), (20).into(), (30).into(),]) }) == 60) as i32) != 0));
    assert!((((({ outer_1(1, &[(42).into(),]) }) == 42) as i32) != 0));
    assert!((((({ outer_1(0, &[]) }) == 0) as i32) != 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
