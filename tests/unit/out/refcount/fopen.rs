extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut fname: Ptr<i8> = Ptr::<i8>::from_string_literal(b"testfile.txt");
    let mut mode: Ptr<i8> = Ptr::<i8>::from_string_literal(b"rb");
    let mut file_ptr: Ptr<CFile> =
        match CFile::open(&fname.to_rust_string(), &mode.to_rust_string()) {
            Some(__f) => Ptr::alloc(__f),
            None => Ptr::null(),
        };
    return 0;
}
pub fn __cpp2rust_init_globals() {}
