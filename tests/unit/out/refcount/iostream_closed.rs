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
    assert!((FdRegistry::close(1) == 0));
    write!(libcc2rs::cout(), "not written\n",);
    write!(libcc2rs::cerr(), "still open\n",);
    assert!((FdRegistry::close(2) == 0));
    write!(libcc2rs::cerr(), "not written either\n",);
    return 0;
}
pub fn __cpp2rust_init_globals() {}
