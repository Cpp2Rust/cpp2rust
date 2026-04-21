extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn deref_0(p: Ptr<i32>) -> i32 {
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(p));
    return ((*p.borrow()).read());
}
pub fn strlen_1(s: Ptr<u8>) -> i32 {
    let s: Value<Ptr<u8>> = Rc::new(RefCell::new(s));
    let c: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while (((*s.borrow_mut()).postfix_inc().read()) != 0) {
        (*c.borrow_mut()).prefix_inc();
    }
    return (*c.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2])));
    let s: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        ('a' as u8),
        ('b' as u8),
        ('c' as u8),
        ('\0' as u8),
    ])));
    return (({
        let _p: Ptr<i32> = (a.as_pointer() as Ptr<i32>);
        deref_0(_p)
    }) + ({
        let _s: Ptr<u8> = (s.as_pointer() as Ptr<u8>);
        strlen_1(_s)
    }));
}
