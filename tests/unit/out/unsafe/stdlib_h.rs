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
    let mut v: *const core::ffi::c_char =
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
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_setenv_getenv_0() });
    return 0;
}
