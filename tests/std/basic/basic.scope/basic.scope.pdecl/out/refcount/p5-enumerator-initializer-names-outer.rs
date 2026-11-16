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
    let mut limit: i32 = 12;
    {
        assert!(((anon_0_limit as i32) == 13));
        assert!(((anon_0_above as i32) == 14));
    }
    assert!((limit == 12));
    return 0;
}
pub type anon_0 = u32;
pub const anon_0_limit: anon_0 = 13;
pub const anon_0_above: anon_0 = 14;
pub fn __cpp2rust_init_globals() {}
