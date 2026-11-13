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
    let mut g: Option<Value<Box<[i32]>>> = Some(Rc::new(RefCell::new(
        (0..2_usize).map(|_| 0_i32).collect::<Box<[_]>>(),
    )));
    g.decay().offset(0_usize).write(11);
    g.decay().offset(1_usize).write(12);
    let mut g_ptr: Ptr<i32> = g.decay();
    elem!(g_ptr, 0).write(13);
    elem!(g_ptr, 1).write(14);
    assert!((((g.decay().offset(0_usize).read()) + (g.decay().offset(1_usize).read())) == 27));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
