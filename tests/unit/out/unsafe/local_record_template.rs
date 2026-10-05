extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn get_4(mut t: Local_0) -> i32 {
    return (t.x as i32);
}
pub unsafe fn get_5(mut t: Local_1) -> i32 {
    return t.x;
}
pub unsafe fn get_6(mut t: Local_2) -> i32 {
    return t.x;
}
pub unsafe fn get_7(mut t: Local_3) -> i32 {
    return (t.x as i32);
}
pub unsafe fn twice_8(mut t: Local_1) -> i32 {
    return ((t.x) * (2));
}
pub unsafe fn wrap_9(mut v: i32) -> i32 {
    let mut l: Local_2 = Local_2 { x: v };
    return (unsafe { get_6(l) });
}
pub unsafe fn wrap_10(mut v: i64) -> i32 {
    let mut l: Local_3 = Local_3 { x: v };
    return (unsafe { get_7(l) });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, Default)]
pub struct Local_2 {
    pub x: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, Default)]
pub struct Local_3 {
    pub x: i64,
}
pub unsafe fn other_11() -> i32 {
    let mut l: Local_0 = Local_0 { x: 3_i64, y: 4_i64 };
    return ((unsafe { get_4(l) }) + (l.y as i32));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, Default)]
pub struct Local_0 {
    pub x: i64,
    pub y: i64,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut l: Local_1 = Local_1 { x: 7 };
    assert!(((unsafe { get_5(l,) }) == (7)));
    assert!(((unsafe { twice_8(l,) }) == (14)));
    assert!(((unsafe { other_11() }) == (7)));
    assert!(((unsafe { wrap_9(5,) }) == (5)));
    assert!(((unsafe { wrap_10(6_i64,) }) == (6)));
    return 0;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, Default)]
pub struct Local_1 {
    pub x: i32,
}
pub unsafe fn __cpp2rust_init_globals() {}
