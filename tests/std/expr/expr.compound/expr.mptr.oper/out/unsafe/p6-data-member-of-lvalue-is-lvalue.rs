extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HasTwoInts {
    pub first: i32,
    pub second: i32,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut pm: isize = (::std::mem::offset_of!(HasTwoInts, second) as isize);
    let mut obj: HasTwoInts = HasTwoInts {
        first: 3,
        second: 4,
    };
    assert!(
        ((&mut (*(&mut obj as *mut HasTwoInts)
            .wrapping_byte_offset(pm)
            .cast::<i32>()) as *mut i32)
            == (&mut obj.second as *mut i32))
    );
    let ref_: *mut i32 = &mut (*(&mut obj as *mut HasTwoInts)
        .wrapping_byte_offset(pm)
        .cast::<i32>());
    (*ref_) = 5;
    assert!(((obj.second) == (5)));
    assert!(((obj.first) == (3)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
