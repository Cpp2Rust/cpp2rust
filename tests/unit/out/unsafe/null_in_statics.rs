extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut p_mut_0: *mut i32 = unsafe { std::ptr::null_mut() };
pub static mut p_const_1: *const i32 = unsafe { std::ptr::null() };
pub static mut cp_2: *const ::core::ffi::c_char = unsafe { std::ptr::null() };
pub static mut arr_of_ptr_3: [*mut i32; 4] = unsafe { [std::ptr::null_mut(); 4] };
pub static mut pp_4: *mut *mut i32 = unsafe { std::ptr::null_mut() };
pub static mut const_arr_of_ptr_5: [*const i32; 3] = unsafe { [std::ptr::null(); 3] };
pub static mut cp_explicit_null_6: *const ::core::ffi::c_char = unsafe { std::ptr::null() };
pub static mut p_zero_7: *mut i32 = unsafe { std::ptr::null_mut() };
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!((p_mut_0).is_null());
    assert!((p_const_1).is_null());
    assert!((cp_2).is_null());
    let mut i: i32 = 0;
    'loop_: while ((i) < (4)) {
        assert!((arr_of_ptr_3[(i) as usize]).is_null());
        i.prefix_inc();
    }
    assert!((pp_4).is_null());
    let mut i: i32 = 0;
    'loop_: while ((i) < (3)) {
        assert!((const_arr_of_ptr_5[(i) as usize]).is_null());
        i.prefix_inc();
    }
    assert!((cp_explicit_null_6).is_null());
    assert!((p_zero_7).is_null());
    return 0;
}
