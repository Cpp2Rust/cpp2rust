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
    assert!((((libc::SOCK_STREAM == 1) as i32) != 0));
    assert!((((libc::SOCK_DGRAM == 2) as i32) != 0));
    return 0;
}
