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
pub struct widget {
    pub id: i32,
}
pub unsafe fn a_value_0() -> i32 {
    let mut w: widget = <widget>::default();
    w.id = 11;
    return w.id;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((((unsafe { a_value_0() }) == (11)) as i32) != 0));
    assert!(((((unsafe { b_value_1() }) == (2)) as i32) != 0));
    return 0;
}
pub type widget_enum = u32;
pub const widget_enum_WIDGET_A: widget_enum = 0;
pub const widget_enum_WIDGET_B: widget_enum = 1;
pub const widget_enum_WIDGET_C: widget_enum = 2;
pub unsafe fn b_value_1() -> i32 {
    let mut w: widget_enum = widget_enum_WIDGET_C;
    return (w as i32);
}
