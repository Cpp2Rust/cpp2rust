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
    let count: Value<u64> = Rc::new(RefCell::new(0_u64));
    'loop_: while (((*s.borrow_mut()).postfix_inc().read()) != 0) {
        (*count.borrow_mut()).prefix_inc();
    }
    return (*count.borrow());
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        ('s' as u8),
        ('t' as u8),
        ('r' as u8),
    ])));
    return (({
        let _s: Ptr<u8> = (s.as_pointer() as Ptr<u8>);
        strlen_0(_s)
    }) as i32);
}
