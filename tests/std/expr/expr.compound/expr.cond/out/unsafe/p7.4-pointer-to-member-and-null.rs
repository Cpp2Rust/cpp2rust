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
pub unsafe fn second_or_null_0(mut choose_second: bool) -> isize {
    return if choose_second {
        (::std::mem::offset_of!(HasTwoInts, second) as isize)
    } else {
        -1_isize
    };
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut obj: HasTwoInts = HasTwoInts {
        first: 3,
        second: 4,
    };
    assert!(((unsafe { second_or_null_0(false,) }) == -1_isize));
    assert!(
        ((unsafe { second_or_null_0(true,) })
            == (::std::mem::offset_of!(HasTwoInts, second) as isize))
    );
    assert!(
        ((*(&mut obj as *mut HasTwoInts)
            .wrapping_byte_offset((unsafe { second_or_null_0(true,) }))
            .cast::<i32>())
            == (4))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
