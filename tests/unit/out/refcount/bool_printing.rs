extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0() -> bool {
    return true;
}
pub fn bar_1() -> bool {
    return true;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut i1: i32 = 0;
    let mut i2: i32 = 1;
    write!(libcc2rs::cout(), "{:}\n", (true as u8),);
    write!(libcc2rs::cout(), "{:}\n", (false as u8),);
    write!(libcc2rs::cout(), "{:}\n", ((i1 != i2) as u8),);
    write!(libcc2rs::cout(), "{:}\n", ((i1 == i2) as u8),);
    write!(libcc2rs::cout(), "{:}\n", (({ foo_0() }) as u8),);
    write!(libcc2rs::cout(), "{:}\n", (({ bar_1() }) as u8),);
    return 0;
}
pub fn __cpp2rust_init_globals() {}
