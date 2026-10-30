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
    assert!(
        (if (0..=255).contains(&(('a' as i8) as i32)) {
            ((('a' as i8) as i32) as u8).to_ascii_uppercase() as i32
        } else {
            (('a' as i8) as i32)
        } == (('A' as i8) as i32))
    );
    assert!(
        (if (0..=255).contains(&(('Z' as i8) as i32)) {
            ((('Z' as i8) as i32) as u8).to_ascii_uppercase() as i32
        } else {
            (('Z' as i8) as i32)
        } == (('Z' as i8) as i32))
    );
    assert!(
        (if (0..=255).contains(&(('5' as i8) as i32)) {
            ((('5' as i8) as i32) as u8).to_ascii_uppercase() as i32
        } else {
            (('5' as i8) as i32)
        } == (('5' as i8) as i32))
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
