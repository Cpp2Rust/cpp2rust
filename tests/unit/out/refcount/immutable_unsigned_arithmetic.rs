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
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<u32> = Rc::new(RefCell::new(0_u32));
    let mut p: Ptr<u32> = (a.as_pointer());
    {
        match ({ (p.read()) }, &(p)) {
            (__tmp_0, __tmp_1) => __tmp_1.with_mut(|__tmp_1| {
                (*__tmp_1) = (((*__tmp_1) as u32).wrapping_add((__tmp_0 as u32))) as u32;
            }),
        }
    };
    return ((p.read()) as i32);
}
pub fn __cpp2rust_init_globals() {}
