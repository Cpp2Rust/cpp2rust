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
    let val: Value<u32> = Rc::new(RefCell::new(42_u32));
    let original: Value<Ptr<u32>> = Rc::new(RefCell::new((val.as_pointer())));
    let as_u16: Value<Ptr<u16>> = Rc::new(RefCell::new(
        ((*original.borrow()).reinterpret_cast::<u16>()).clone(),
    ));
    let back: Value<Ptr<u32>> = Rc::new(RefCell::new(
        ((*as_u16.borrow()).reinterpret_cast::<u32>()).clone(),
    ));
    assert!({
        let _lhs = (*back.borrow()).clone();
        _lhs == (*original.borrow()).clone()
    });
    assert!((((*back.borrow()).read()) == 42_u32));
    let as_u8: Value<Ptr<u8>> = Rc::new(RefCell::new(
        ((*original.borrow()).reinterpret_cast::<u8>()).clone(),
    ));
    let back2: Value<Ptr<u32>> = Rc::new(RefCell::new(
        ((*as_u8.borrow()).reinterpret_cast::<u32>()).clone(),
    ));
    assert!({
        let _lhs = (*back2.borrow()).clone();
        _lhs == (*original.borrow()).clone()
    });
    assert!((((*back2.borrow()).read()) == 42_u32));
    return 0;
}
