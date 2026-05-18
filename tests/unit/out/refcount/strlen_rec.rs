extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn strlen_0(s: Ptr<u8>, n: i32) -> i32 {
    let s: Value<Ptr<u8>> = Rc::new(RefCell::new(s));
    let n: Value<i32> = Rc::new(RefCell::new(n));
    return if (((*s.borrow()).read()) != 0) {
        ({
            let _s: Ptr<u8> = (*s.borrow()).offset((1) as isize);
            let _n: i32 = ((*n.borrow()) + 1);
            strlen_0(_s, _n)
        })
    } else {
        (*n.borrow())
    };
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        ('s' as u8),
        ('t' as u8),
        ('r' as u8),
        ('\0' as u8),
    ])));
    return ({
        let _s: Ptr<u8> = ((s.as_pointer() as Ptr<u8>).offset(0 as isize));
        let _n: i32 = 0;
        strlen_0(_s, _n)
    });
}
