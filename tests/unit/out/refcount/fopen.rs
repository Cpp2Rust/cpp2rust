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
    let fname: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal("testfile.txt")));
    let mode: Value<Ptr<u8>> = Rc::new(RefCell::new(Ptr::from_string_literal("rb")));
    let file_ptr: Value<Ptr<::std::fs::File>> =
        Rc::new(RefCell::new(match (*mode.borrow()).to_rust_string() {
            v if v == "rb" => std::fs::OpenOptions::new()
                .read(true)
                .open((*fname.borrow()).to_rust_string())
                .ok()
                .map_or(Ptr::null(), |f| Ptr::alloc(f)),
            v if v == "wb" => std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open((*fname.borrow()).to_rust_string())
                .ok()
                .map_or(Ptr::null(), |f| Ptr::alloc(f)),
            _ => panic!("unsupported mode"),
        }));
    return 0;
}
