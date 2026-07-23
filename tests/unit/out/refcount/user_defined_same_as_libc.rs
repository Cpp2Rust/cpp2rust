extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fopen_0(path: Ptr<u8>, mode: Ptr<u8>) -> Ptr<CFile> {
    let path: Value<Ptr<u8>> = Rc::new(RefCell::new(path));
    let mode: Value<Ptr<u8>> = Rc::new(RefCell::new(mode));
    (*path.borrow()).clone();
    (*mode.borrow()).clone();
    return Ptr::null();
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let fp: Value<Ptr<CFile>> = Rc::new(RefCell::new(
        ({
            fopen_0(
                Ptr::from_string_literal(b"/tmp/irrelevant-file"),
                Ptr::from_string_literal(b"r"),
            )
        }),
    ));
    assert!(((((*fp.borrow()).is_null()) as i32) != 0));
    return 0;
}
