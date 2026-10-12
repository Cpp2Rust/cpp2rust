extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn extract_nth_0(mut n: i32, ap: Ptr<VaArg>) -> i32 {
    let ap: Value<Ptr<VaArg>> = Rc::new(RefCell::new(ap));
    let mut i: i32 = 0;
    'loop_: while (((i < n) as i32) != 0) {
        (*ap.borrow_mut()).arg::<i32>();
        i.postfix_inc();
    }
    return (*ap.borrow_mut()).arg::<i32>();
}
pub fn middle_layer_1(mut n: i32, ap: Ptr<VaArg>) -> i32 {
    let ap: Value<Ptr<VaArg>> = Rc::new(RefCell::new(ap));
    return ({ extract_nth_0(n, (*ap.borrow()).clone()) });
}
pub fn top_level_2(n: i32, __args: &[VaArg]) -> i32 {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let ap: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
    (*ap.borrow_mut()) = __args.as_pointer();
    let mut result: i32 = ({ middle_layer_1((*n.borrow()), (*ap.borrow()).clone()) });
    return result;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (((({
            top_level_2(2, &[(100).into(), (200).into(), (300).into(), (400).into()])
        }) == 300) as i32)
            != 0)
    );
    assert!((((({ top_level_2(0, &[(42).into(), (99).into(),]) }) == 42) as i32) != 0));
    assert!(
        (((({ top_level_2(3, &[(1).into(), (2).into(), (3).into(), (4).into(),]) }) == 4) as i32)
            != 0)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
