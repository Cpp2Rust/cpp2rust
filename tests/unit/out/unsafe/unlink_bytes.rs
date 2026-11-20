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
    fp = libc::fopen(
        (c"unlink-\xff.tmp".as_ptr().cast_mut()).cast_const(),
        (c"wb".as_ptr().cast_mut()).cast_const(),
    );
    assert!((!((fp).is_null())));
    assert!(((libc::fclose(fp)) == (0)));
    let mut st: ::libc::stat = std::mem::zeroed();
    assert!(
        (((libc::stat(
            (c"unlink-\xff.tmp".as_ptr().cast_mut()).cast_const(),
            (&mut st as *mut ::libc::stat)
        )) == (0))
            && ((st.st_size) == (0 as ::libc::off_t)))
    );
    assert!(
        (((libc::stat(sibling, (&mut st as *mut ::libc::stat))) == (0))
            && ((st.st_size) == (8 as ::libc::off_t)))
    );
    assert!(
        ((libc::rename(
            (c"unlink-\xff.tmp".as_ptr().cast_mut()).cast_const(),
            (c"unlink-\xfe.tmp".as_ptr().cast_mut()).cast_const()
        )) == (0))
    );
    let mut fd: i32 = (unsafe {
        libc::open(
            (c"unlink-\xfe.tmp".as_ptr().cast_mut()).cast_const() as *const i8,
            ::libc::O_RDONLY as i32,
        )
    });
    assert!(((fd) >= (0)));
    assert!(((libc::close(fd)) == (0)));
    let mut path: *mut libc::c_char = libc::realpath(
        (c"unlink-\xfe.tmp".as_ptr().cast_mut()).cast_const(),
        std::ptr::null_mut(),
    );
    assert!((!((path).is_null())));
    assert!(
        ((libc::strcmp(
            (libc::strrchr((path).cast_const(), ('/' as i32))).cast_const(),
            (c"/unlink-\xfe.tmp".as_ptr().cast_mut()).cast_const()
        )) == (0))
    );
    libcc2rs::free_unsafe((path as *mut ::libc::c_void));
    assert!(((libc::unlink((c"unlink-\xfe.tmp".as_ptr().cast_mut()).cast_const())) == (0)));
    assert!(
        (((libc::stat(sibling, (&mut st as *mut ::libc::stat))) == (0))
            && ((st.st_size) == (8 as ::libc::off_t)))
    );
    assert!(((libc::mkdir((c"unlink-\xfd".as_ptr().cast_mut()).cast_const(), 493_u32)) == (0)));
    let mut dir: *mut ::libc::DIR =
        libc::opendir((c"unlink-\xfd".as_ptr().cast_mut()).cast_const());
    assert!((!((dir).is_null())));
    assert!(((libc::closedir(dir)) == (0)));
    assert!(((libc::rmdir((c"unlink-\xfd".as_ptr().cast_mut()).cast_const())) == (0)));
    assert!(((libc::unlink(sibling)) == (0)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
