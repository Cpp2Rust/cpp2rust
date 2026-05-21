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
            (b"CPP2RUST_TEST_VAR\0".as_ptr().cast_mut()).cast_const() as *const i8,
            (b"test_value\0".as_ptr().cast_mut()).cast_const() as *const i8,
            1
        )) == (0)) as i32)
            != 0)
    );
    let mut v: *const u8 =
        (libc::getenv((b"CPP2RUST_TEST_VAR\0".as_ptr().cast_mut()).cast_const() as *const i8)
            as *mut u8)
            .cast_const();
    assert!((((!((v).is_null())) as i32) != 0));
    assert!(
        ((((libc::strcmp(
            v as *const i8,
            (b"test_value\0".as_ptr().cast_mut()).cast_const() as *const i8
        )) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((libc::setenv(
            (b"CPP2RUST_TEST_VAR\0".as_ptr().cast_mut()).cast_const() as *const i8,
            (b"replaced\0".as_ptr().cast_mut()).cast_const() as *const i8,
            1
        )) == (0)) as i32)
            != 0)
    );
    v = (libc::getenv((b"CPP2RUST_TEST_VAR\0".as_ptr().cast_mut()).cast_const() as *const i8)
        as *mut u8)
        .cast_const();
    assert!((((!((v).is_null())) as i32) != 0));
    assert!(
        ((((libc::strcmp(
            v as *const i8,
            (b"replaced\0".as_ptr().cast_mut()).cast_const() as *const i8
        )) == (0)) as i32)
            != 0)
    );
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_setenv_getenv_0() });
    return 0;
}
