extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct S {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: bool,
}
impl S {
    pub fn new() -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self { a: 11, b: true }));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for S {
    fn default() -> Self {
        { S::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Declared {
    #[offset(0)]
    pub v: i32,
}
impl Declared {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut d: Ptr<Declared> = Ptr::<Declared>::null();
    assert!((d).is_null());
    let s: Value<S> = Rc::new(RefCell::new(S::new()));
    assert!(({ (*s.borrow()).a } == 11));
    assert!((({ (*s.borrow()).b } as i32) == (true as i32)));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
