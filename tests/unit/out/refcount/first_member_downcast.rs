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
pub struct base {
    #[offset(0)]
    pub kind: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct derived {
    #[offset(0)]
    #[byte_size(4)]
    pub head: base,
    #[offset(8)]
    pub value: usize,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut d: Ptr<derived> = libcc2rs::malloc_refcount(16usize).reinterpret_cast::<derived>();
    assert!((((!((d).is_null())) as i32) != 0));
    field!(field!(d, head), kind).write(3);
    field!(d, value).write(7_usize);
    let mut b: Ptr<base> = (field_ptr!(d, head));
    let mut back: Ptr<derived> = b.reinterpret_cast::<derived>();
    assert!(((({ (back).clone() } == { (d).clone() }) as i32) != 0));
    assert!((((back.with(|__s| __s.value) == 7_usize) as i32) != 0));
    assert!((((back.with(|__s| __s.head.kind) == 3) as i32) != 0));
    field!(back, value).write(8_usize);
    assert!((((d.with(|__s| __s.value) == 8_usize) as i32) != 0));
    field!(b, kind).write(4);
    assert!((((d.with(|__s| __s.head.kind) == 4) as i32) != 0));
    libcc2rs::free_refcount((back).to_any());
    return 0;
}
pub fn __cpp2rust_init_globals() {}
