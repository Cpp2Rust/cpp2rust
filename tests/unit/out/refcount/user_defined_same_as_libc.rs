extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fopen_0(path: Ptr<u8>, mode: Ptr<u8>) -> Ptr<::std::fs::File> {
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
    let fp: Value<Ptr<::std::fs::File>> = Rc::new(RefCell::new(
        ({
            let _path: Ptr<u8> = Ptr::from_string_literal("/tmp/irrelevant-file");
            let _mode: Ptr<u8> = Ptr::from_string_literal("r");
            fopen_0(_path, _mode)
        }),
    ));
    assert!(((((*fp.borrow()).is_null()) as i32) != 0));
    return 0;
}
