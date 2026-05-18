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
pub struct iter {
    pub p: *mut i32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct const_iter {
    pub p: *const i32,
}
impl const_iter {
    pub unsafe fn const_iter(o: *const iter) -> Self {
        let mut this = Self {
            p: (*o).p.cast_const(),
        };
        this
    }
}
pub unsafe fn sink_0(mut i: const_iter) {}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut buf: [i32; 2] = [0, 0];
    let mut it: iter = iter {
        p: buf.as_mut_ptr(),
    };
    (unsafe {
        let _i: const_iter = const_iter::const_iter(&it as *const iter);
        sink_0(_i)
    });
    return 0;
}
