extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut p_mut_0: std::cell::LazyCell<*mut i32> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null_mut() });
pub static mut p_const_1: std::cell::LazyCell<*const i32> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null() });
pub static mut cp_2: std::cell::LazyCell<*const libc::c_char> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null() });
pub static mut arr_of_ptr_3: std::cell::LazyCell<[*mut i32; 4]> =
    std::cell::LazyCell::new(|| unsafe { [std::ptr::null_mut(); 4] });
pub static mut pp_4: std::cell::LazyCell<*mut *mut i32> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null_mut() });
pub static mut const_arr_of_ptr_5: std::cell::LazyCell<[*const i32; 3]> =
    std::cell::LazyCell::new(|| unsafe { [std::ptr::null(); 3] });
pub static mut cp_explicit_null_6: std::cell::LazyCell<*const libc::c_char> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null() });
pub static mut p_zero_7: std::cell::LazyCell<*mut i32> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null_mut() });
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!((*std::cell::LazyCell::force_mut(&mut *&raw mut p_mut_0)).is_null());
    assert!((*std::cell::LazyCell::force_mut(&mut *&raw mut p_const_1)).is_null());
    assert!((*std::cell::LazyCell::force_mut(&mut *&raw mut cp_2)).is_null());
    let mut i: i32 = 0;
    'loop_: while ((i) < (4)) {
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut arr_of_ptr_3))[(i) as usize])
                .is_null()
        );
        i.prefix_inc();
    }
    assert!((*std::cell::LazyCell::force_mut(&mut *&raw mut pp_4)).is_null());
    let mut i: i32 = 0;
    'loop_: while ((i) < (3)) {
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut const_arr_of_ptr_5))[(i) as usize])
                .is_null()
        );
        i.prefix_inc();
    }
    assert!((*std::cell::LazyCell::force_mut(&mut *&raw mut cp_explicit_null_6)).is_null());
    assert!((*std::cell::LazyCell::force_mut(&mut *&raw mut p_zero_7)).is_null());
    return 0;
}
