extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut sibling: *const libc::c_char =
        (c"unlink-\xef\xbf\xbd.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut ::libc::FILE = libc::fopen(sibling, (c"wb".as_ptr().cast_mut()).cast_const());
    assert!((!((fp).is_null())));
    assert!(((libc::fputs((c"sentinel".as_ptr().cast_mut()).cast_const(), fp)) >= (0)));
    assert!(((libc::fclose(fp)) == (0)));
    (*libcc2rs::cpp2rust_errno_unsafe()) = 0;
    assert!(((libc::unlink((c"unlink-\xff.tmp".as_ptr().cast_mut()).cast_const())) == (-1_i32)));
    assert!(((*libcc2rs::cpp2rust_errno_unsafe()) == (libc::ENOENT)));
    fp = libc::fopen(sibling, (c"rb".as_ptr().cast_mut()).cast_const());
    assert!((!((fp).is_null())));
    assert!(((libc::fclose(fp)) == (0)));
    assert!(((libc::unlink(sibling)) == (0)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
