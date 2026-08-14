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
    let b1: Value<u8> = Rc::new(RefCell::new(1_u8));
    let ishift1: Value<i32> = Rc::new(RefCell::new(3));
    let shl1: Value<u8> = Rc::new(RefCell::new((*b1.borrow()) << (*ishift1.borrow())));
    assert!(((*shl1.borrow()) == 8));
    let ishift2: Value<i32> = Rc::new(RefCell::new(2));
    let shr1: Value<u8> = Rc::new(RefCell::new((*shl1.borrow()) >> (*ishift2.borrow())));
    assert!(((*shr1.borrow()) == 2));
    let ishift3: Value<i32> = Rc::new(RefCell::new(5));
    {
        let n_ = (*b1.borrow()) << (*ishift3.borrow());
        (*b1.borrow_mut()) = n_;
        (*b1.borrow())
    };
    assert!(((*b1.borrow()) == 32));
    let ishift4: Value<i32> = Rc::new(RefCell::new(3));
    {
        let n_ = (*b1.borrow()) >> (*ishift4.borrow());
        (*b1.borrow_mut()) = n_;
        (*b1.borrow())
    };
    assert!(((*b1.borrow()) == 4));
    let b2: Value<u8> = Rc::new(RefCell::new(1_u8));
    let ushift1: Value<u32> = Rc::new(RefCell::new(3_u32));
    let shl2: Value<u8> = Rc::new(RefCell::new((*b2.borrow()) << (*ushift1.borrow())));
    assert!(((*shl2.borrow()) == 8));
    let ushift2: Value<u32> = Rc::new(RefCell::new(2_u32));
    let shr2: Value<u8> = Rc::new(RefCell::new((*shl2.borrow()) >> (*ushift2.borrow())));
    assert!(((*shr2.borrow()) == 2));
    let ushift3: Value<u32> = Rc::new(RefCell::new(5_u32));
    {
        let n_ = (*b2.borrow()) << (*ushift3.borrow());
        (*b2.borrow_mut()) = n_;
        (*b2.borrow())
    };
    assert!(((*b2.borrow()) == 32));
    let ushift4: Value<u32> = Rc::new(RefCell::new(3_u32));
    {
        let n_ = (*b2.borrow()) >> (*ushift4.borrow());
        (*b2.borrow_mut()) = n_;
        (*b2.borrow())
    };
    assert!(((*b2.borrow()) == 4));
    return 0;
}
