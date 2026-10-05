extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn strlen_0(s: Ptr<i8>) -> usize {
    let s: Value<Ptr<i8>> = Rc::new(RefCell::new(s));
    let begin: Value<Ptr<i8>> = Rc::new(RefCell::new((*s.borrow()).clone()));
    'loop_: while (((*s.borrow()).read()) != 0) {
        (*s.borrow_mut()).prefix_inc();
    }
    return ((((*s.borrow()).clone() - (*begin.borrow()).clone()) as i64) as usize);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        ('s' as i8),
        ('t' as i8),
        ('r' as i8),
        ('i' as i8),
        ('n' as i8),
        ('g' as i8),
        ('\0' as i8),
    ])));
    assert!((({ strlen_0(((s.as_pointer() as Ptr<i8>).offset(0)),) }) == 6_usize));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
