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
pub struct container {
    pub p: *mut opaque,
    pub x: i32,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut c: container = container {
        p: std::ptr::null_mut(),
        x: 42,
    };
    (unsafe {
        let _c: *mut container = (&mut c as *mut container);
        touch_0(_c)
    });
    assert!(((((c.x) == (42)) as i32) != 0));
    assert!(((((c.p).is_null()) as i32) != 0));
    return 0;
}
pub unsafe fn touch_0(mut c: *mut container) {
    &((*c).p);
}
#[repr(C)]
pub struct opaque {
    _opaque: [u8; 0],
}
