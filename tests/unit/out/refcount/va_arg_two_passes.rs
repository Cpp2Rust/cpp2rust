extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sum_then_product_0(first: i32, __args: &[VaArg]) -> i32 {
    let first: Value<i32> = Rc::new(RefCell::new(first));
    let ap: Value<VaList> = Rc::new(RefCell::new(VaList::default()));
    let mut sum: i32 = (*first.borrow());
    let mut product: i32 = (*first.borrow());
    (*ap.borrow_mut()) = VaList::new(__args);
    let mut val: i32 = 0_i32;
    'loop_: while (((({
        val = (*ap.borrow_mut()).arg::<i32>();
        val
    }) != 0) as i32)
        != 0)
    {
        sum += val;
    }
    (*ap.borrow_mut()) = VaList::new(__args);
    'loop_: while (((({
        val = (*ap.borrow_mut()).arg::<i32>();
        val
    }) != 0) as i32)
        != 0)
    {
        product *= val;
    }
    return (sum + product);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (((({ sum_then_product_0(2, &[(3).into(), (4).into(), (0).into(),]) }) == 33) as i32) != 0)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
