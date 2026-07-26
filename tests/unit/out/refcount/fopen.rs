extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let fname: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(b"testfile.txt")));
    let mode: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal(b"rb")));
    let file_ptr: Value<Ptr<CFile>> = Rc::new(RefCell::new(
        match CFile::open(
            &(*fname.borrow()).to_rust_string(),
            &(*mode.borrow()).to_rust_string(),
        ) {
            Some(__f) => Ptr::alloc(__f),
            None => Ptr::null(),
        },
    ));
    return 0;
}
