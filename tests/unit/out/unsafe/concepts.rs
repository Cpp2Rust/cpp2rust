extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Sized {}
impl Sized {
    pub unsafe fn size(&mut self) -> i32 {
        return 4;
    }
}
pub unsafe fn is_small_0() -> bool {
    return true;
}
pub unsafe fn is_small_1() -> bool {
    return false;
}
pub unsafe fn has_size_2() -> bool {
    return true;
}
pub unsafe fn has_size_3() -> bool {
    return false;
}
pub unsafe fn pick_4(mut x: i32) -> i32 {
    if (true) && (true) {
        return 1;
    }
    return 2;
}
pub unsafe fn pick_5(mut x: i64) -> i32 {
    if (true) && (false) {
        return 1;
    }
    return 2;
}
pub unsafe fn pick_6(mut x: f32) -> i32 {
    if (false) && (true) {
        return 1;
    }
    return 2;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!((unsafe { is_small_0() }));
    assert!(!(unsafe { is_small_1() }));
    assert!(true);
    assert!(!false);
    assert!((unsafe { has_size_2() }));
    assert!(!(unsafe { has_size_3() }));
    assert!(((unsafe { pick_4(1,) }) == (1)));
    assert!(((unsafe { pick_5(1_i64,) }) == (2)));
    assert!(((unsafe { pick_6(1.0E+0,) }) == (2)));
    return 0;
}
