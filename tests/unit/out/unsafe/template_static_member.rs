extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut s_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 55 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Static_int_ {}
pub static mut s_1: std::cell::LazyCell<libc::c_char> =
    std::cell::LazyCell::new(|| unsafe { (55 as libc::c_char) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Static_char_ {}
pub static mut s_2: std::cell::LazyCell<i64> = std::cell::LazyCell::new(|| unsafe { 55_i64 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Static_long_ {}
pub static mut lo_3: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { -1_i32 });
pub static mut hi_4: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 1 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Range_int_ {}
pub static mut lo_5: std::cell::LazyCell<i64> =
    std::cell::LazyCell::new(|| unsafe { (-2_i32 as i64) });
pub static mut hi_6: std::cell::LazyCell<i64> = std::cell::LazyCell::new(|| unsafe { 2_i64 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Range_long_ {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut s_0)) = 22;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut s_1)) = (33 as libc::c_char);
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut s_0)) == (22)));
    assert!((((*std::cell::LazyCell::force_mut(&mut *&raw mut s_1)) as i32) == (33)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut s_2)) == (55_i64)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut lo_3)) == (-1_i32))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut hi_4)) == (1))
    );
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut lo_5)) == (-2_i32 as i64))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut hi_6)) == (2_i64))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const s_0);
    std::cell::LazyCell::force(&*&raw const s_1);
    std::cell::LazyCell::force(&*&raw const s_2);
    std::cell::LazyCell::force(&*&raw const lo_3);
    std::cell::LazyCell::force(&*&raw const hi_4);
    std::cell::LazyCell::force(&*&raw const lo_5);
    std::cell::LazyCell::force(&*&raw const hi_6);
}
