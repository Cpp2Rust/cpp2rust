extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn mixed_args_0(count: i32, args: &[VaArg]) -> i32 {
    let count: Value<i32> = Rc::new(RefCell::new(count));
    let ap: Value<VaList> = Rc::new(RefCell::new(VaList::default()));
    (*ap.borrow_mut()) = VaList::new(args);
    let total: Value<i32> = Rc::new(RefCell::new(0));
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < (*count.borrow())) {
        let tag: Value<i32> = Rc::new(RefCell::new(((*ap.borrow_mut()).arg::<i32>()).clone()));
        if ((*tag.borrow()) == 0) {
            (*total.borrow_mut()) += ((*ap.borrow_mut()).arg::<i32>()).clone();
        } else {
            let ptr: Value<Ptr<i32>> =
                Rc::new(RefCell::new(((*ap.borrow_mut()).arg::<Ptr<i32>>()).clone()));
            let __rhs = ((*ptr.borrow()).read());
            (*total.borrow_mut()) += __rhs;
        }
        (*i.borrow_mut()).postfix_inc();
    }
    return (*total.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(100));
    assert!(
        (({
            let _count: i32 = 3;
            mixed_args_0(
                _count,
                &[
                    0.into(),
                    10.into(),
                    1.into(),
                    (x.as_pointer()).into(),
                    0.into(),
                    20.into(),
                ],
            )
        }) == 130)
    );
    let y: Value<i32> = Rc::new(RefCell::new(50));
    assert!(
        (({
            let _count: i32 = 1;
            mixed_args_0(_count, &[1.into(), (y.as_pointer()).into()])
        }) == 50)
    );
    assert!(
        (({
            let _count: i32 = 2;
            mixed_args_0(_count, &[0.into(), 5.into(), 0.into(), 3.into()])
        }) == 8)
    );
    return 0;
}
