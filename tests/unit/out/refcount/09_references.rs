extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let h: Value<i32> = Rc::new(RefCell::new(15));
    let h_ref1: Ptr<i32> = h.as_pointer();
    h_ref1.write(16);
    let h_ptr: Value<Ptr<i32>> = Rc::new(RefCell::new((h_ref1).clone()));
    let h_ref2: Ptr<i32> = (*h_ptr.borrow()).clone();
    h_ref2.write(17);
    assert!(
        ({
            let _lhs = (h_ref1.read());
            _lhs + (h_ref2.read())
        } == 34)
    );
    let a: Value<i32> = Rc::new(RefCell::new(1));
    let b: Value<i32> = Rc::new(RefCell::new(2));
    let r: Ptr<i32> = if ((*a.borrow()) < (*b.borrow())) {
        a.as_pointer()
    } else {
        b.as_pointer()
    };
    r.write(10);
    assert!(((*a.borrow()) == 10));
    let cr: Ptr<i32> = if ((*a.borrow()) > (*b.borrow())) {
        a.as_pointer()
    } else {
        b.as_pointer()
    };
    assert!(((cr.read()) == 10));
    let x: Value<i32> = Rc::new(RefCell::new(1));
    let y: Value<i32> = Rc::new(RefCell::new(2));
    let cx: Ptr<i32> = if ((*x.borrow()) < (*y.borrow())) {
        x.as_pointer()
    } else {
        y.as_pointer()
    };
    assert!(((cx.read()) == 1));
    let cp: Value<Ptr<i32>> = Rc::new(RefCell::new(if ((*a.borrow()) > (*b.borrow())) {
        (a.as_pointer())
    } else {
        (b.as_pointer())
    }));
    assert!((((*cp.borrow()).read()) == 10));
    let mp: Value<Ptr<i32>> = Rc::new(RefCell::new(if ((*a.borrow()) < (*b.borrow())) {
        (a.as_pointer())
    } else {
        (b.as_pointer())
    }));
    (*mp.borrow()).write(20);
    assert!(((*b.borrow()) == 20));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
