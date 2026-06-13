extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn get_greeting_0() -> *const core::ffi::c_char {
    return c"hello".as_ptr();
}
pub unsafe fn get_empty_1() -> *const core::ffi::c_char {
    return c"".as_ptr();
}
pub unsafe fn get_branch_2(mut x: i32) -> *const core::ffi::c_char {
    if ((x) > (0)) {
        return c"positive".as_ptr();
    }
    return c"non-positive".as_ptr();
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: *const core::ffi::c_char = (unsafe { get_greeting_0() });
    assert!((((*a.offset((0) as isize)) as i32) == (('h' as core::ffi::c_char) as i32)));
    assert!((((*a.offset((4) as isize)) as i32) == (('o' as core::ffi::c_char) as i32)));
    assert!((((*a.offset((5) as isize)) as i32) == (('\0' as core::ffi::c_char) as i32)));
    let mut b: *const core::ffi::c_char = (unsafe { get_empty_1() });
    assert!((((*b.offset((0) as isize)) as i32) == (('\0' as core::ffi::c_char) as i32)));
    let mut c: *const core::ffi::c_char = (unsafe { get_branch_2(1) });
    assert!((((*c.offset((0) as isize)) as i32) == (('p' as core::ffi::c_char) as i32)));
    assert!((((*c.offset((7) as isize)) as i32) == (('e' as core::ffi::c_char) as i32)));
    assert!((((*c.offset((8) as isize)) as i32) == (('\0' as core::ffi::c_char) as i32)));
    let mut d: *const core::ffi::c_char = (unsafe {
        let _x: i32 = -1_i32;
        get_branch_2(_x)
    });
    assert!((((*d.offset((0) as isize)) as i32) == (('n' as core::ffi::c_char) as i32)));
    assert!((((*d.offset((11) as isize)) as i32) == (('e' as core::ffi::c_char) as i32)));
    assert!((((*d.offset((12) as isize)) as i32) == (('\0' as core::ffi::c_char) as i32)));
    return 0;
}
