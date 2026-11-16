extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static count_0: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct HasStaticCounter {}
impl HasStaticCounter {
    pub fn bump() {
        (*count_0.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ HasStaticCounter::bump() });
    assert!((count_0.with(|rc| *rc.borrow()) == 1));
    ({ HasStaticCounter::bump() });
    assert!((count_0.with(|rc| *rc.borrow()) == 2));
    count_0.with(|rc| *rc.borrow_mut() = 6);
    assert!((count_0.with(|rc| *rc.borrow()) == 6));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = count_0.with(|_| ());
}
