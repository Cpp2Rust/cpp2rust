extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn deref_0(mut p: Ptr<i32>) -> i32 {
    return (p.read());
}
pub fn strlen_1(mut s: Ptr<i8>) -> i32 {
    let mut c: i32 = 0;
    'loop_: while ((s.postfix_inc().read()) != 0) {
        c.prefix_inc();
    }
    return c;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2])));
    let s: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        ('a' as i8),
        ('b' as i8),
        ('c' as i8),
        ('\0' as i8),
    ])));
    assert!(
        ((({ deref_0((a.as_pointer() as Ptr::<i32>),) })
            + ({ strlen_1((s.as_pointer() as Ptr::<i8>),) }))
            == 4)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
