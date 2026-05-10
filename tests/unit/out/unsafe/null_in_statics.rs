extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut p_mut: *mut i32 = std::ptr::null_mut();
pub static mut p_const: *const i32 = std::ptr::null();
pub static mut cp: *const u8 = std::ptr::null();
pub static mut arr_of_ptr: [*mut i32; 4] = [std::ptr::null_mut(); 4];
pub static mut pp: *mut *mut i32 = std::ptr::null_mut();
pub static mut const_arr_of_ptr: [*const i32; 3] = [std::ptr::null(); 3];
pub static mut cp_explicit_null: *const u8 = std::ptr::null();
pub static mut p_zero: *mut i32 = std::ptr::null_mut();
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!((p_mut).is_null());
    assert!((p_const).is_null());
    assert!((cp).is_null());
    let mut i: i32 = 0;
    'loop_: while ((i) < (4)) {
        assert!((arr_of_ptr[(i) as usize]).is_null());
        i.prefix_inc();
    }
    assert!((pp).is_null());
    let mut i: i32 = 0;
    'loop_: while ((i) < (3)) {
        assert!((const_arr_of_ptr[(i) as usize]).is_null());
        i.prefix_inc();
    }
    assert!((cp_explicit_null).is_null());
    assert!((p_zero).is_null());
    return 0;
}
