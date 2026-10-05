extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn strlen_0(mut s: Ptr<i8>) -> usize {
    let mut count: usize = 0_usize;
    'loop_: while ((s.postfix_inc().read()) != 0) {
        count.prefix_inc();
    }
    return count;
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
    ])));
    return (({ strlen_0((s.as_pointer() as Ptr<i8>)) }) as i32);
}
pub fn __cpp2rust_init_globals() {}
