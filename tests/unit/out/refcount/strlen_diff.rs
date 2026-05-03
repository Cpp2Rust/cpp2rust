extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn strlen_0(s: Ptr<u8>) -> u64 {
    let s: Value<Ptr<u8>> = Rc::new(RefCell::new(s));
    let begin: Value<Ptr<u8>> = Rc::new(RefCell::new((*s.borrow()).clone()));
    'loop_: while (((*s.borrow()).read()) != 0) {
        (*s.borrow_mut()).prefix_inc();
    }
    return ((((*s.borrow()).clone() - (*begin.borrow()).clone()) as i64) as u64);
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        ('s' as u8),
        ('t' as u8),
        ('r' as u8),
        ('i' as u8),
        ('n' as u8),
        ('g' as u8),
        ('\0' as u8),
    ])));
    return (({
        let _s: Ptr<u8> = ((s.as_pointer() as Ptr<u8>).offset(0 as isize));
        strlen_0(_s)
    }) as i32);
}
