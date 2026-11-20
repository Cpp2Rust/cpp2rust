extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_getenv_0() {
    assert!(
        ((libc::getenv((c"CPP2RUST_DEFINITELY_MISSING_VAR".as_ptr().cast_mut()).cast_const()))
            .is_null())
    );
}
pub unsafe fn test_realpath_1() {
    let mut buf: [libc::c_char; 4096] = [(0 as libc::c_char); 4096];
    assert!(
        (!((libc::realpath((c"/".as_ptr().cast_mut()).cast_const(), buf.as_mut_ptr())).is_null()))
    );
    assert!(
        ((libc::strcmp(
            (buf.as_mut_ptr()).cast_const(),
            (c"/".as_ptr().cast_mut()).cast_const()
        )) == (0))
    );
    let mut p: *mut libc::c_char = libc::realpath(
        (c"/".as_ptr().cast_mut()).cast_const(),
        std::ptr::null_mut(),
    );
    assert!((!((p).is_null())));
    assert!(((libc::strcmp((p).cast_const(), (c"/".as_ptr().cast_mut()).cast_const())) == (0)));
    libcc2rs::free_unsafe((p as *mut ::libc::c_void));
    (*libcc2rs::cpp2rust_errno_unsafe()) = 0;
    assert!(
        ((libc::realpath(
            (c"/cpp2rust_definitely_missing".as_ptr().cast_mut()).cast_const(),
            buf.as_mut_ptr()
        ))
        .is_null())
    );
    assert!(((*libcc2rs::cpp2rust_errno_unsafe()) == (libc::ENOENT)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_getenv_0() });
    (unsafe { test_realpath_1() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
