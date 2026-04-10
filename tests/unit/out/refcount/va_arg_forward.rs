extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn inner_0(count: i32, ap: VaList) -> i32 {
    let count: Value<i32> = Rc::new(RefCell::new(count));
    let ap: Value<VaList> = Rc::new(RefCell::new(ap));
    let total: Value<i32> = Rc::new(RefCell::new(0));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*count.borrow())) {
        (*total.borrow_mut()) += ((*ap.borrow_mut()).arg::<i32>()).clone();
        (*i.borrow_mut()).postfix_inc();
    }
    return (*total.borrow());
}
pub fn outer_1(count: i32, args: &[VaArg]) -> i32 {
    let count: Value<i32> = Rc::new(RefCell::new(count));
    let ap: Value<VaList> = Rc::new(RefCell::new(<VaList>::default()));
    (*ap.borrow_mut()) = VaList::new(args);
    let result: Value<i32> = Rc::new(RefCell::new(
        ({
            let _count: i32 = (*count.borrow());
            let _ap: VaList = (*ap.borrow()).clone();
            inner_0(_count, _ap)
        }),
    ));
    return (*result.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (({
            let _count: i32 = 3;
            outer_1(_count, &[10.into(), 20.into(), 30.into()])
        }) == 60)
    );
    assert!(
        (({
            let _count: i32 = 1;
            outer_1(_count, &[42.into()])
        }) == 42)
    );
    assert!(
        (({
            let _count: i32 = 0;
            outer_1(_count, &[])
        }) == 0)
    );
    return 0;
}
