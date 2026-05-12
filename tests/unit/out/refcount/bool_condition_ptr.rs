extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let storage: Value<i32> = Rc::new(RefCell::new(7));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new((storage.as_pointer())));
    let np: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::<i32>::null()));
    if !(*p.borrow()).is_null() {
        assert!(true);
    }
    if !!(*p.borrow()).is_null() {
        assert!(false);
    }
    if !(*np.borrow()).is_null() {
        assert!(false);
    }
    if !!(*np.borrow()).is_null() {
        assert!(true);
    }
    let iter: Value<Ptr<i32>> = Rc::new(RefCell::new((*p.borrow()).clone()));
    let iters: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while !(*iter.borrow()).is_null() {
        (*iters.borrow_mut()).prefix_inc();
        (*iter.borrow_mut()) = Ptr::<i32>::null();
    }
    assert!(((*iters.borrow()) == 1));
    let t3: Value<i32> = Rc::new(RefCell::new(if !(*p.borrow()).is_null() { 1 } else { 0 }));
    assert!(((*t3.borrow()) == 1));
    let t4: Value<i32> = Rc::new(RefCell::new(if !(*np.borrow()).is_null() { 1 } else { 0 }));
    assert!(((*t4.borrow()) == 0));
    let t5: Value<i32> = Rc::new(RefCell::new((!!(*p.borrow()).is_null() as i32)));
    assert!(((*t5.borrow()) == 0));
    let t6: Value<i32> = Rc::new(RefCell::new((!!(*np.borrow()).is_null() as i32)));
    assert!(((*t6.borrow()) == 1));
    let b2: Value<bool> = Rc::new(RefCell::new((!(*p.borrow()).is_null()).clone()));
    let b3: Value<bool> = Rc::new(RefCell::new((!(*np.borrow()).is_null()).clone()));
    assert!((*b2.borrow()));
    assert!(!(*b3.borrow()));
    return 0;
}
