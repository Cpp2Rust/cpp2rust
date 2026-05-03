extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sum_ints_0(first: i32, args: &[VaArg]) -> i32 {
    let first: Value<i32> = Rc::new(RefCell::new(first));
    let ap: Value<VaList> = Rc::new(RefCell::new(VaList::default()));
    let total: Value<i32> = Rc::new(RefCell::new((*first.borrow())));
    (*ap.borrow_mut()) = VaList::new(args);
    let val: Value<i32> = <Value<i32>>::default();
    'loop_: while (((({
        (*val.borrow_mut()) = ((*ap.borrow_mut()).arg::<i32>()).clone();
        (*val.borrow())
    }) != 0) as i32)
        != 0)
    {
        (*total.borrow_mut()) += (*val.borrow());
    }
    return (*total.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (((({
            let _first: i32 = 1;
            sum_ints_0(_first, &[2.into(), 3.into(), 4.into(), 0.into()])
        }) == 10) as i32)
            != 0)
    );
    assert!(
        (((({
            let _first: i32 = 100;
            sum_ints_0(_first, &[0.into()])
        }) == 100) as i32)
            != 0)
    );
    assert!(
        (((({
            let _first: i32 = 5;
            sum_ints_0(_first, &[5.into(), 5.into(), 5.into(), 5.into(), 0.into()])
        }) == 25) as i32)
            != 0)
    );
    return 0;
}
