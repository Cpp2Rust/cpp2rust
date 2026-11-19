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
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let mut level: u8 = 12_u8;
    {
        let mut level: u8 = 0_u8;
        level = level;
        return (level as i32);
    }
    0
}
pub fn __cpp2rust_init_globals() {}
