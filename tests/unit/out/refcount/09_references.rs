extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static counter_0: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn inc_1() {
    counter_0.with(|rc| *rc.borrow_mut() += 1);
}
pub fn dec_2() {
    counter_0.with(|rc| *rc.borrow_mut() -= 1);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let h: Value<i32> = Rc::new(RefCell::new(15));
    let h_ref1: Ptr<i32> = h.as_pointer();
    h_ref1.write(16);
    let mut h_ptr: Ptr<i32> = (h_ref1).clone();
    let h_ref2: Ptr<i32> = (h_ptr).clone();
    h_ref2.write(17);
    assert!((({ (h_ref1.read()) } + { (h_ref2.read()) }) == 34));
    let a: Value<i32> = Rc::new(RefCell::new(1));
    let b: Value<i32> = Rc::new(RefCell::new(2));
    let r: Ptr<i32> = if ((*a.borrow()) < (*b.borrow())) {
        (a.as_pointer())
    } else {
        (b.as_pointer())
    };
    r.write(10);
    assert!(((*a.borrow()) == 10));
    let cr: Ptr<i32> = if ((*a.borrow()) > (*b.borrow())) {
        (a.as_pointer())
    } else {
        (b.as_pointer())
    };
    assert!(((cr.read()) == 10));
    let x: Value<i32> = Rc::new(RefCell::new(1));
    let y: Value<i32> = Rc::new(RefCell::new(2));
    let cx: Ptr<i32> = if ((*x.borrow()) < (*y.borrow())) {
        (x.as_pointer())
    } else {
        (y.as_pointer())
    };
    assert!(((cx.read()) == 1));
    let mut cp: Ptr<i32> = if ((*a.borrow()) > (*b.borrow())) {
        (a.as_pointer())
    } else {
        (b.as_pointer())
    };
    assert!(((cp.read()) == 10));
    let mut mp: Ptr<i32> = if ((*a.borrow()) < (*b.borrow())) {
        (a.as_pointer())
    } else {
        (b.as_pointer())
    };
    mp.write(20);
    assert!(((*b.borrow()) == 20));
    if ((*a.borrow()) < (*b.borrow())) {
        (a.as_pointer())
    } else {
        (b.as_pointer())
    }
    .write(30);
    assert!(((*a.borrow()) == 30));
    assert!(((*b.borrow()) == 20));
    {
        if ((*a.borrow()) < (*b.borrow())) {
            (a.as_pointer())
        } else {
            (b.as_pointer())
        }
        .with_mut(|__v| *__v = *__v + 5)
    };
    assert!(((*b.borrow()) == 25));
    let mut ap: Ptr<i32> = (if ((*a.borrow()) > (*b.borrow())) {
        (a.as_pointer())
    } else {
        (b.as_pointer())
    });
    ap.write(40);
    assert!(((*a.borrow()) == 40));
    if ((*a.borrow()) < (*b.borrow())) {
        ({ inc_1() });
    } else {
        ({ dec_2() });
    };
    assert!((counter_0.with(|rc| *rc.borrow()) == -1_i32));
    if ((*a.borrow()) > (*b.borrow())) {
        ({ inc_1() });
    } else {
        ({ dec_2() });
    };
    assert!((counter_0.with(|rc| *rc.borrow()) == 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = counter_0.with(|_| ());
}
