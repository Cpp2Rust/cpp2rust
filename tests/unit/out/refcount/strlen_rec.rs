extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn strlen_0(mut s: Ptr<i8>, mut n: i32) -> i32 {
    return if ((s.read()) != 0) {
        ({ strlen_0(s.offset((1) as isize), (n + 1)) })
    } else {
        n
    };
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
        ('\0' as i8),
    ])));
    assert!((({ strlen_0(((s.as_pointer() as Ptr<i8>).offset(0)), 0,) }) == 3));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
