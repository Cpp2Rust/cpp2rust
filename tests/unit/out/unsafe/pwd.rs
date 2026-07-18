extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_getpwuid_0() {
    let mut pw: *mut ::libc::passwd = libc::getpwuid(libc::geteuid());
    assert!((((!((pw).is_null())) as i32) != 0));
    assert!((((((*pw).pw_uid) == (libc::geteuid())) as i32) != 0));
    assert!(((((libc::strlen(((*pw).pw_name).cast_const())) > (0_usize)) as i32) != 0));
    assert!((((!(((*pw).pw_dir).is_null())) as i32) != 0));
    printf(
        (c"%s\n".as_ptr().cast_mut()).cast_const() as *const i8,
        (*pw).pw_name,
    );
}
pub unsafe fn test_getpwuid_missing_1() {
    (*libcc2rs::cpp2rust_errno_unsafe()) = 0;
    let mut pw: *mut ::libc::passwd = libc::getpwuid(2147483646_u32);
    assert!(((((pw).is_null()) as i32) != 0));
    assert!(((((*libcc2rs::cpp2rust_errno_unsafe()) == (0)) as i32) != 0));
}
pub unsafe fn test_getpwuid_r_2() {
    let mut pw: ::libc::passwd = unsafe { std::mem::zeroed() };
    let mut buf: [libc::c_char; 4096] = [(0 as libc::c_char); 4096];
    let mut result: *mut ::libc::passwd = std::ptr::null_mut();
    assert!(
        ((((libc::getpwuid_r(
            libc::geteuid(),
            (&mut pw as *mut ::libc::passwd),
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 4096]>(),
            (&mut result as *mut *mut ::libc::passwd)
        )) == (0)) as i32)
            != 0)
    );
    assert!(((((result) == (&mut pw as *mut ::libc::passwd)) as i32) != 0));
    assert!(((((pw.pw_uid) == (libc::geteuid())) as i32) != 0));
    assert!(((((libc::strlen((pw.pw_name).cast_const())) > (0_usize)) as i32) != 0));
    let mut pw2: *mut ::libc::passwd = libc::getpwuid(libc::geteuid());
    assert!((((!((pw2).is_null())) as i32) != 0));
    assert!(
        ((((libc::strcmp((pw.pw_name).cast_const(), ((*pw2).pw_name).cast_const())) == (0))
            as i32)
            != 0)
    );
    printf(
        (c"%s\n".as_ptr().cast_mut()).cast_const() as *const i8,
        pw.pw_name,
    );
}
pub unsafe fn test_getpwuid_r_erange_3() {
    let mut pw: ::libc::passwd = unsafe { std::mem::zeroed() };
    let mut tiny: [libc::c_char; 1] = [(0 as libc::c_char); 1];
    let mut result: *mut ::libc::passwd = std::ptr::null_mut();
    assert!(
        ((((libc::getpwuid_r(
            libc::geteuid(),
            (&mut pw as *mut ::libc::passwd),
            tiny.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1]>(),
            (&mut result as *mut *mut ::libc::passwd)
        )) == (34)) as i32)
            != 0)
    );
    assert!(((((result).is_null()) as i32) != 0));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_getpwuid_0() });
    (unsafe { test_getpwuid_missing_1() });
    (unsafe { test_getpwuid_r_2() });
    (unsafe { test_getpwuid_r_erange_3() });
    return 0;
}
