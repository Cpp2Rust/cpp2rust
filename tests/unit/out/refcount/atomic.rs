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
pub struct Counter {
    #[offset(0)]
    pub n: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<Counter> = Rc::new(RefCell::new(Counter::default()));
    let mut v: Counter = ((*a.borrow()) as Counter);
    assert!((v.n == 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
