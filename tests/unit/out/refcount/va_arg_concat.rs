extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sum_ints_0(first: i32, __args: &[VaArg]) -> i32 {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    let first: Value<i32> = Rc::new(RefCell::new(first));
    let args: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
    let mut total: i32 = (*first.borrow());
    (*args.borrow_mut()) = __args.as_pointer();
    let mut val: i32 = 0_i32;
    'loop_: while (((({
        val = (*args.borrow_mut()).arg::<i32>();
        val
    }) != 0) as i32)
        != 0)
    {
        total += val;
    }
    return total;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (((({ sum_ints_0(1, &[(2).into(), (3).into(), (4).into(), (0).into(),]) }) == 10) as i32)
            != 0)
    );
    assert!((((({ sum_ints_0(100, &[(0).into(),]) }) == 100) as i32) != 0));
    assert!(
        (((({
            sum_ints_0(
                5,
                &[(5).into(), (5).into(), (5).into(), (5).into(), (0).into()],
            )
        }) == 25) as i32)
            != 0)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
