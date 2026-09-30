extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct container {
    #[offset(0)]
    #[byte_size(8)]
    pub p: Ptr<opaque>,
    #[offset(8)]
    pub x: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let c: Value<container> = Rc::new(RefCell::new(container {
        p: Ptr::<opaque>::null(),
        x: 42,
    }));
    &({ (*c.borrow()).p.clone() });
    return ({ (*c.borrow()).x } - 42);
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct opaque;
pub fn __cpp2rust_init_globals() {}
