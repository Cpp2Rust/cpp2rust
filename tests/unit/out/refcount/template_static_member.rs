extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static s_0: Value<i32> = Rc::new(RefCell::new(55));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Static_int_ {}
thread_local!(
    pub static s_1: Value<u8> = Rc::new(RefCell::new(55_u8));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Static_char_ {}
thread_local!(
    pub static s_2: Value<i64> = Rc::new(RefCell::new(55_i64));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Static_long_ {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    s_0.with(|rc| *rc.borrow_mut() = 22);
    s_1.with(|rc| *rc.borrow_mut() = 33_u8);
    assert!((s_0.with(|rc| *rc.borrow()) == 22));
    assert!(((s_1.with(|rc| *rc.borrow()) as i32) == 33));
    assert!((s_2.with(|rc| *rc.borrow()) == 55_i64));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = s_0.with(|_| ());
    let _ = s_1.with(|_| ());
    let _ = s_2.with(|_| ());
}
