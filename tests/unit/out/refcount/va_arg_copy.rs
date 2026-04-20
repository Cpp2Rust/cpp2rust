extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sum_with_copy_0(count: i32, args: &[VaArg]) -> i32 {
    let count: Value<i32> = Rc::new(RefCell::new(count));
    let ap: Value<VaList> = Rc::new(RefCell::new(VaList::default()));
    let aq: Value<VaList> = Rc::new(RefCell::new(VaList::default()));
    (*ap.borrow_mut()) = VaList::new(args);
    (*aq.borrow_mut()) = (*ap.borrow_mut()).clone();
    let sum1: Value<i32> = Rc::new(RefCell::new(0));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*count.borrow())) {
        (*sum1.borrow_mut()) += ((*ap.borrow_mut()).arg::<i32>()).clone();
        (*i.borrow_mut()).postfix_inc();
    }
    let sum2: Value<i32> = Rc::new(RefCell::new(0));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*count.borrow())) {
        (*sum2.borrow_mut()) += ((*aq.borrow_mut()).arg::<i32>()).clone();
        (*i.borrow_mut()).postfix_inc();
    }
    assert!(((*sum1.borrow()) == (*sum2.borrow())));
    return ((*sum1.borrow()) + (*sum2.borrow()));
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (({
            let _count: i32 = 3;
            sum_with_copy_0(_count, &[10.into(), 20.into(), 30.into()])
        }) == 120)
    );
    return 0;
}
