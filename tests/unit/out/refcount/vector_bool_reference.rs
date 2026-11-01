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
    let v1: Value<Vec<bool>> = Rc::new(RefCell::new(Vec::from([true])));
    let mut r1: Ptr<bool> = v1.decay().offset(0_usize as isize);
    assert!((v1.decay().offset(0_usize as isize).read()) && (r1.read()));
    {
        r1.write(false);
        (r1).clone()
    };
    assert!(!(v1.decay().offset(0_usize as isize).read()));
    let it: Value<RandomAccessIterator<bool>> = Rc::new(RefCell::new(v1.decay().begin()));
    let mut r2: Ptr<bool> = (*it.borrow()).iter_deref();
    {
        r2.write(true);
        (r2).clone()
    };
    assert!(v1.decay().offset(0_usize as isize).read());
    match (Rc::new(RefCell::new(v1.decay().offset(0_usize as isize))).as_pointer(),) {
        (__tmp_0,) => {
            (*__tmp_0.upgrade().deref()).write(false);
            (*__tmp_0.upgrade().deref()).clone()
        }
    };
    assert!(!(v1.decay().offset(0_usize as isize).read()));
    {
        let v: Value<Vec<bool>> = Rc::new(RefCell::new(Vec::from([false, false, false])));
        let mut i: i32 = 0;
        let mut b: bool = match (Rc::new(RefCell::new(
            v.decay().offset((i.postfix_inc() as usize) as isize),
        ))
        .as_pointer(),)
        {
            (__tmp_1,) => {
                (*__tmp_1.upgrade().deref()).write(true);
                (*__tmp_1.upgrade().deref()).clone()
            }
        }
        .read();
        assert!((b) && (i == 1));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
