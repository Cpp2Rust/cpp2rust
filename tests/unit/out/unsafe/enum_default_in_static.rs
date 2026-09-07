extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type Mode = u32;
pub const Mode_MODE_NONE: Mode = 0;
pub const Mode_MODE_ONE: Mode = 1;
pub const Mode_MODE_TWO: Mode = 2;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Config {
    pub count: i32,
    pub mode: Mode,
}
pub static mut config_0: Config = unsafe {
    Config {
        count: 0_i32,
        mode: Mode_MODE_NONE,
    }
};
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((((config_0.count) == (0)) as i32) != 0));
    assert!(((((config_0.mode as u32) == ((Mode_MODE_NONE as i32) as u32)) as i32) != 0));
    return 0;
}
