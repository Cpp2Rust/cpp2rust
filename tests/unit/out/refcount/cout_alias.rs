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
    let mut cond: bool = true;
    let mut os1: Ptr<std::fs::File> = if cond {
        (libcc2rs::cout())
    } else {
        (libcc2rs::cerr())
    };
    write!(os1, "hello\n",);
    let os2: Ptr<std::fs::File> = if cond {
        (libcc2rs::cout())
    } else {
        (libcc2rs::cerr())
    };
    write!(os2, "hello\n",);
    let is: Ptr<std::fs::File> = libcc2rs::cin();
    assert!((is.with_mut(libcc2rs::istream_get) == -1_i32));
    let mut is_ptr: Ptr<std::fs::File> = (libcc2rs::cin());
    assert!((is_ptr.with_mut(libcc2rs::istream_get) == -1_i32));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
