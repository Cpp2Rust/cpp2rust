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
    let arr: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([1_u8, 2_u8, 3_u8])));
    let p: Value<Ptr<u8>> = Rc::new(RefCell::new(((arr.as_pointer() as Ptr<u8>).offset(0))));
    (*p.borrow_mut()) += (1_u8 as i32);
    assert!(((((*p.borrow()).read()) as i32) == 2));
    let p: Value<Ptr<u8>> = Rc::new(RefCell::new(((arr.as_pointer() as Ptr<u8>).offset(0))));
    (*p.borrow_mut()) += (1_i8 as i32);
    assert!(((((*p.borrow()).read()) as i32) == 2));
    let p: Value<Ptr<u8>> = Rc::new(RefCell::new(((arr.as_pointer() as Ptr<u8>).offset(0))));
    (*p.borrow_mut()) += (1_u16 as i32);
    assert!(((((*p.borrow()).read()) as i32) == 2));
    let p: Value<Ptr<u8>> = Rc::new(RefCell::new(((arr.as_pointer() as Ptr<u8>).offset(0))));
    (*p.borrow_mut()) += (1_i16 as i32);
    assert!(((((*p.borrow()).read()) as i32) == 2));
    let p: Value<Ptr<u8>> = Rc::new(RefCell::new(((arr.as_pointer() as Ptr<u8>).offset(0))));
    (*p.borrow_mut()) += 1_u32;
    assert!(((((*p.borrow()).read()) as i32) == 2));
    let p: Value<Ptr<u8>> = Rc::new(RefCell::new(((arr.as_pointer() as Ptr<u8>).offset(0))));
    (*p.borrow_mut()) += (1 as i32);
    assert!(((((*p.borrow()).read()) as i32) == 2));
    let p: Value<Ptr<u8>> = Rc::new(RefCell::new(((arr.as_pointer() as Ptr<u8>).offset(0))));
    (*p.borrow_mut()) += 1_u64;
    assert!(((((*p.borrow()).read()) as i32) == 2));
    let p: Value<Ptr<u8>> = Rc::new(RefCell::new(((arr.as_pointer() as Ptr<u8>).offset(0))));
    (*p.borrow_mut()) += 1_i64;
    assert!(((((*p.borrow()).read()) as i32) == 2));
    return 0;
}
