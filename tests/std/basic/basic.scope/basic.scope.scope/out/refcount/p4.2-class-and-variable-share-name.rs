extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Limit {
    #[offset(0)]
    pub value: i32,
}
thread_local!(
    pub static Limit_0: Value<i32> = Rc::new(RefCell::new(12));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut limit: Limit = Limit { value: 3 };
    assert!((limit.value == 3));
    assert!((Limit_0.with(|rc| *rc.borrow()) == 12));
    Limit_0.with(|rc| *rc.borrow_mut() += limit.value);
    assert!((Limit_0.with(|rc| *rc.borrow()) == 15));
    assert!((limit.value == 3));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = Limit_0.with(|_| ());
}
