extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type Code = u32;
pub const Code_CODE_OK: Code = 0;
pub const Code_CODE_ERR: Code = 1;
pub const Code_CODE_FATAL: Code = 2;
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut code: Code = Code_CODE_OK;
    let mut err: Code = Code_CODE_ERR;
    if (code != 0) {
        assert!((0 != 0));
    }
    if !(code != 0) {
        assert!((1 != 0));
    }
    if (err != 0) {
        assert!((1 != 0));
    }
    if !(err != 0) {
        assert!((0 != 0));
    }
    let mut t9: i32 = (!(code != 0) as i32);
    assert!(((((t9) == (1)) as i32) != 0));
    let mut b4: bool = (code != 0);
    assert!(((!b4 as i32) != 0));
    return 0;
}
