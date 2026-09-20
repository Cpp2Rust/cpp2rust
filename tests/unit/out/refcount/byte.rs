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
    let b1: Value<u8> = Rc::new(RefCell::new(1_u8));
    let ushift1: Value<u32> = Rc::new(RefCell::new(3_u32));
    let shl1: Value<u8> = Rc::new(RefCell::new((*b1.borrow()) << (*ushift1.borrow())));
    assert!(((*shl1.borrow()) == ((8) as u8)));
    let ushift2: Value<u32> = Rc::new(RefCell::new(2_u32));
    let shr1: Value<u8> = Rc::new(RefCell::new((*shl1.borrow()) >> (*ushift2.borrow())));
    assert!(((*shr1.borrow()) == ((2) as u8)));
    let ushift3: Value<u32> = Rc::new(RefCell::new(5_u32));
    {
        let n_ = (*b1.borrow()) << (*ushift3.borrow());
        (*b1.borrow_mut()) = n_;
        (*b1.borrow())
    };
    assert!(((*b1.borrow()) == ((32) as u8)));
    let ushift4: Value<u32> = Rc::new(RefCell::new(3_u32));
    {
        let n_ = (*b1.borrow()) >> (*ushift4.borrow());
        (*b1.borrow_mut()) = n_;
        (*b1.borrow())
    };
    assert!(((*b1.borrow()) == ((4) as u8)));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
