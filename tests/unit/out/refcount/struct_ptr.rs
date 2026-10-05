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
pub struct XX {
    #[offset(0)]
    pub x: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let obj: Value<XX> = Rc::new(RefCell::new(<XX>::default()));
    let mut ptr: Ptr<XX> = (obj.as_pointer());
    field!(ptr, x).write(2);
    let mut c: bool = false;
    let mut r: i32 = if c {
        { (*obj.borrow()).x }
    } else {
        ptr.with(|__s| __s.x)
    };
    let mut p: Ptr<i32> = (field_ptr!(obj.as_pointer(), x));
    assert!((({ (p.read()) } + { r }) == 4));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
