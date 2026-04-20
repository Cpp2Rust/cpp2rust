extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::Seek;
use std::io::{Read, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(0));
    let b: Value<i32> = <Value<i32>>::default();
    if ({
        (*b.borrow_mut()) = (*a.borrow());
        (*b.borrow())
    } != 0)
    {}
    'loop_: while ((({
        (*b.borrow_mut()) = (*a.borrow());
        (*b.borrow())
    }) as i32)
        != 0)
    {}
    if ((*a.borrow()) != 0) {}
    if ((*a.borrow()) == (*b.borrow())) {}
    if ((*a.borrow()) < (*b.borrow())) {}
    assert!(((*a.borrow()) == (*b.borrow())));
    assert!(
        (!(({
            (*a.borrow_mut()) = (*b.borrow());
            (*a.borrow())
        }) as i32)
            != 0)
    );
    let c: Value<bool> = <Value<bool>>::default();
    (*c.borrow_mut()) = ({
        (*a.borrow_mut()) = (*b.borrow());
        (*a.borrow())
    } != 0);
    (*c.borrow_mut()) = ((({
        (*b.borrow_mut()) = (*a.borrow());
        (*b.borrow())
    }) as i32)
        != 0);
    (*c.borrow_mut()) = ((*a.borrow()) != 0);
    (*c.borrow_mut()) = ((*a.borrow()) == (*b.borrow()));
    (*c.borrow_mut()) = ((*a.borrow()) < (*b.borrow()));
    return 0;
}
