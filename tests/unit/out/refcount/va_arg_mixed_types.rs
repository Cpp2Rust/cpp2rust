extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sum_mixed_0(count: i32, args: &[VaArg]) -> f64 {
    let count: Value<i32> = Rc::new(RefCell::new(count));
    let ap: Value<VaList> = Rc::new(RefCell::new(<VaList>::default()));
    (*ap.borrow_mut()) = VaList::new(args);
    let total: Value<f64> = Rc::new(RefCell::new(0_f64));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*count.borrow())) {
        (*total.borrow_mut()) += ((*ap.borrow_mut()).arg::<f64>()).clone();
        (*i.borrow_mut()).postfix_inc();
    }
    return (*total.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    return ((({
        let _count: i32 = 3;
        sum_mixed_0(_count, &[1.5E+0.into(), 2.5E+0.into(), 3.0E+0.into()])
    }) as i32)
        - 7);
}
