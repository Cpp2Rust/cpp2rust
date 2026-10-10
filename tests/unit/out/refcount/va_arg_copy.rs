extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sum_with_copy_0(count: i32, __args: &[VaArg]) -> i32 {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    let count: Value<i32> = Rc::new(RefCell::new(count));
    let ap: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
    let aq: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
    (*ap.borrow_mut()) = __args.as_pointer();
    (*aq.borrow_mut()) = (*ap.borrow_mut()).clone();
    let mut sum1: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while (((i < (*count.borrow())) as i32) != 0) {
        sum1 += (*ap.borrow_mut()).arg::<i32>();
        i.postfix_inc();
    }
    let mut sum2: i32 = 0;
    let mut i: i32 = 0;
    'loop_: while (((i < (*count.borrow())) as i32) != 0) {
        sum2 += (*aq.borrow_mut()).arg::<i32>();
        i.postfix_inc();
    }
    assert!((((sum1 == sum2) as i32) != 0));
    return (sum1 + sum2);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (((({ sum_with_copy_0(3, &[(10).into(), (20).into(), (30).into(),]) }) == 120) as i32)
            != 0)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
