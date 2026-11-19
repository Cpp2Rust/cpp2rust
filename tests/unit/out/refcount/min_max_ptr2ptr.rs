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
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(10));
    let b: Value<i32> = Rc::new(RefCell::new(20));
    let pa: Value<Ptr<i32>> = Rc::new(RefCell::new((a.as_pointer())));
    let pb: Value<Ptr<i32>> = Rc::new(RefCell::new((b.as_pointer())));
    let mut ppa: Ptr<Ptr<i32>> = (pa.as_pointer());
    let mut ppb: Ptr<Ptr<i32>> = (pb.as_pointer());
    let mut r1: i32 = (if CppLess::less(&(ppa.read()), &(ppb.read())) {
        (ppb.read())
    } else {
        (ppa.read())
    }
    .read());
    let mut r2: i32 = (if CppLess::less(&(ppb.read()), &(ppa.read())) {
        (ppb.read())
    } else {
        (ppa.read())
    }
    .read());
    assert!(((r1 + r2) == 30));
    let values: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([2, 1])));
    let low: Value<Ptr<i32>> = Rc::new(RefCell::new((values.decay().offset(0))));
    let high: Value<Ptr<i32>> = Rc::new(RefCell::new((values.decay().offset(1))));
    assert!(
        ({
            ({
                (if CppLess::less(&high.as_pointer(), &low.as_pointer()) {
                    high.as_pointer()
                } else {
                    low.as_pointer()
                }
                .read())
            } == { (*low.borrow()).clone() })
        })
    );
    assert!(
        ({
            ({
                (if CppLess::less(&low.as_pointer(), &high.as_pointer()) {
                    low.as_pointer()
                } else {
                    high.as_pointer()
                }
                .read())
            } == { (*low.borrow()).clone() })
        })
    );
    assert!(
        ({
            ({
                (if CppLess::less(&low.as_pointer(), &high.as_pointer()) {
                    high.as_pointer()
                } else {
                    low.as_pointer()
                }
                .read())
            } == { (*high.borrow()).clone() })
        })
    );
    assert!(
        ({
            ({
                (if CppLess::less(&high.as_pointer(), &low.as_pointer()) {
                    low.as_pointer()
                } else {
                    high.as_pointer()
                }
                .read())
            } == { (*high.borrow()).clone() })
        })
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
