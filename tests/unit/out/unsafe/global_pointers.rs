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
pub struct Entry {
    pub name: *const u8,
    pub p: *mut i32,
}
pub static mut single_entry: Entry = unsafe {
    Entry {
        name: b"alone\0".as_ptr(),
        p: std::ptr::null_mut(),
    }
};
pub static mut entries: [Entry; 2] = unsafe {
    [
        Entry {
            name: b"first\0".as_ptr(),
            p: std::ptr::null_mut(),
        },
        Entry {
            name: b"second\0".as_ptr(),
            p: std::ptr::null_mut(),
        },
    ]
};
pub static mut arr_of_pointers: [*mut u8; 3] = unsafe {
    [
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    ]
};
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!((single_entry.p).is_null());
    let mut i: i32 = 0;
    'loop_: while ((i) < (2)) {
        assert!((entries[(i) as usize].p).is_null());
        assert!((arr_of_pointers[(i) as usize]).is_null());
        i.prefix_inc();
    }
    return 0;
}
