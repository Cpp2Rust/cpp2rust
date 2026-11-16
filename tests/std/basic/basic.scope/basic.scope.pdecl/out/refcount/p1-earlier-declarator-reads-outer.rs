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
    let mut level: i32 = 5;
    {
        let mut saved: i32 = level;
        let mut level: i32 = (saved + 1);
        let mut next: i32 = (level + 1);
        assert!((saved == 5));
        assert!((level == 6));
        assert!((next == 7));
    }
    assert!((level == 5));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
