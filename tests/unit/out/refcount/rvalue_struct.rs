extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct S {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: i32,
}
impl S {
    pub fn new(mut a: i32, mut b: i32) -> Self {
        Self { a: a, b: b }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s1: Value<S> = Rc::new(RefCell::new(S::new({ 1 }, { 2 })));
    let s2: Ptr<S> = s1.as_pointer();
    assert!((s2.with(|__s| __s.a) == 1));
    assert!((s2.with(|__s| __s.b) == 2));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
