extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fopen_0(path: Ptr<i8>, mode: Ptr<i8>) -> Ptr<CFile> {
    let path: Value<Ptr<i8>> = Rc::new(RefCell::new(path));
    let mode: Value<Ptr<i8>> = Rc::new(RefCell::new(mode));
    &(*path.borrow());
    &(*mode.borrow());
    return Ptr::null();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let fp: Value<Ptr<CFile>> = Rc::new(RefCell::new(
        ({
            fopen_0(
                Ptr::<i8>::from_string_literal(b"irrelevant-file"),
                Ptr::<i8>::from_string_literal(b"r"),
            )
        }),
    ));
    assert!(((((*fp.borrow()).is_null()) as i32) != 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
