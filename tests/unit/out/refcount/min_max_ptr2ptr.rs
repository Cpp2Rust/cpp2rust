extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<i32> = Rc::new(RefCell::new(10));
    let b: Value<i32> = Rc::new(RefCell::new(20));
    let pa: Value<Ptr<i32>> = Rc::new(RefCell::new((a.as_pointer())));
    let pb: Value<Ptr<i32>> = Rc::new(RefCell::new((b.as_pointer())));
    let mut ppa: Ptr<Ptr<i32>> = (pa.as_pointer());
    let mut ppb: Ptr<Ptr<i32>> = (pb.as_pointer());
    let mut r1: i32 = (if (ppa.read()).clone().read() >= (ppb.read()).clone().read() {
        (ppa.read()).clone()
    } else {
        (ppb.read()).clone()
    }
    .read());
    let mut r2: i32 = (if (ppa.read()).clone().read() <= (ppb.read()).clone().read() {
        (ppa.read()).clone()
    } else {
        (ppb.read()).clone()
    }
    .read());
    assert!(((r1 + r2) == 30));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
