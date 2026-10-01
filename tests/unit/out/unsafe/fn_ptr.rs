extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn my_foo_0(mut p: *mut ::libc::c_void) -> i32 {
    return (*(p as *mut i32));
}
pub unsafe fn foo_1(
    mut fn_: Option<unsafe fn(*mut ::libc::c_void) -> i32>,
    mut pi: *mut i32,
) -> i32 {
    return (unsafe { (fn_).unwrap()((pi as *mut ::libc::c_void)) });
}
pub unsafe fn twice_2(mut x: u64) -> u64 {
    return (x).wrapping_mul(2_u64);
}
pub unsafe fn twice_in_place_3(x: *mut u64) -> u64 {
    (*x) = (*x).wrapping_mul(2_u64);
    return (*x);
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut fn_: Option<unsafe fn(*mut ::libc::c_void) -> i32> = None;
    assert!((fn_).is_none());
    assert!(((fn_) != (Some(my_foo_0))));
    fn_ = Some(my_foo_0);
    assert!(!((fn_).is_none()));
    assert!(((fn_) == (Some(my_foo_0))));
    let mut a: i32 = 10;
    assert!(((unsafe { foo_1(fn_, (&mut a as *mut i32),) }) == (a)));
    let mut ul_fn: Option<unsafe fn(u64) -> u64> = (Some(twice_2));
    let mut n: usize = 21_usize;
    let mut r: usize = ((unsafe { (ul_fn).unwrap()((n as u64)) }) as usize);
    assert!(((r) == (42_usize)));
    let mut ul_ref_fn: Option<unsafe fn(*mut u64) -> u64> = (Some(twice_in_place_3));
    let mut m: usize = 21_usize;
    let mut q: usize =
        ((unsafe { (ul_ref_fn).unwrap()((&mut m as *mut usize).cast::<u64>()) }) as usize);
    assert!(((q) == (42_usize)));
    assert!(((m) == (42_usize)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
