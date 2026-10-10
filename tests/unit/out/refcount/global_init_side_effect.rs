extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static total_0: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct S {}
impl S {
    pub fn new(mut x: i32) -> Self {
        let __this: S = Self {};
        total_0.with(|rc| *rc.borrow_mut() += x);
        __this
    }
}
thread_local!(
    pub static a_1: Value<S> = Rc::new(RefCell::new(S::new({ 1 })));
);
thread_local!(
    pub static b_2: Value<S> = Rc::new(RefCell::new(S::new({ 10 })));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((total_0.with(|rc| *rc.borrow()) == 11));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = total_0.with(|_| ());
    let _ = a_1.with(|_| ());
    let _ = b_2.with(|_| ());
}
