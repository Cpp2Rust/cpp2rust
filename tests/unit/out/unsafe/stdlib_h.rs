extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_setenv_getenv_0() {
    assert!(
        ((((libc::setenv(
            (c"CPP2RUST_TEST_VAR".as_ptr().cast_mut()).cast_const(),
            (c"test_value".as_ptr().cast_mut()).cast_const(),
            1
        )) == (0)) as i32)
            != 0)
    );
    let mut v: *const libc::c_char =
        (libc::getenv((c"CPP2RUST_TEST_VAR".as_ptr().cast_mut()).cast_const())).cast_const();
    assert!((((!((v).is_null())) as i32) != 0));
    assert!(
        ((((libc::strcmp(v, (c"test_value".as_ptr().cast_mut()).cast_const())) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((libc::setenv(
            (c"CPP2RUST_TEST_VAR".as_ptr().cast_mut()).cast_const(),
            (c"replaced".as_ptr().cast_mut()).cast_const(),
            1
        )) == (0)) as i32)
            != 0)
    );
    v = (libc::getenv((c"CPP2RUST_TEST_VAR".as_ptr().cast_mut()).cast_const())).cast_const();
    assert!((((!((v).is_null())) as i32) != 0));
    assert!(
        ((((libc::strcmp(v, (c"replaced".as_ptr().cast_mut()).cast_const())) == (0)) as i32) != 0)
    );
}
pub unsafe fn test_realpath_1() {
    let mut buf: [libc::c_char; 4096] = [(0 as libc::c_char); 4096];
    assert!(
        (((!((libc::realpath((c"/".as_ptr().cast_mut()).cast_const(), buf.as_mut_ptr())).is_null()))
            as i32)
            != 0)
    );
    assert!(
        ((((libc::strcmp(
            (buf.as_mut_ptr()).cast_const(),
            (c"/".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    let mut p: *mut libc::c_char = libc::realpath(
        (c"/".as_ptr().cast_mut()).cast_const(),
        std::ptr::null_mut(),
    );
    assert!((((!((p).is_null())) as i32) != 0));
    assert!(
        ((((libc::strcmp((p).cast_const(), (c"/".as_ptr().cast_mut()).cast_const())) == (0))
            as i32)
            != 0)
    );
    libcc2rs::free_unsafe((p as *mut libc::c_char as *mut ::libc::c_void));
    (*libcc2rs::cpp2rust_errno_unsafe()) = 0;
    assert!(
        ((((libc::realpath(
            (c"/cpp2rust_definitely_missing".as_ptr().cast_mut()).cast_const(),
            buf.as_mut_ptr()
        ))
        .is_null()) as i32)
            != 0)
    );
    assert!(((((*libcc2rs::cpp2rust_errno_unsafe()) == (2)) as i32) != 0));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_setenv_getenv_0() });
    (unsafe { test_realpath_1() });
    return 0;
}
