extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn write_ulong_0(p: Ptr<u64>) {
    let p: Value<Ptr<u64>> = Rc::new(RefCell::new(p));
    (*p.borrow()).write(42_u64);
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<usize> = Rc::new(RefCell::new(0_usize));
    ({ write_ulong_0((x.as_pointer().reinterpret_cast::<u64>())) });
    assert!(((((*x.borrow()) == 42_usize) as i32) != 0));
    return 0;
}
