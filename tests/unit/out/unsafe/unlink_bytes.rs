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
    let mut buf: [libc::c_char; 8] = [(0 as libc::c_char); 8];
    assert!(
        ((libcc2rs::fread_unsafe(
            (buf.as_mut_ptr() as *mut ::libc::c_void),
            1_usize,
            ::std::mem::size_of::<[libc::c_char; 8]>(),
            fp
        )) == (::std::mem::size_of::<[libc::c_char; 8]>()))
    );
    assert!(
        (({
            let sa = core::slice::from_raw_parts(
                (buf.as_mut_ptr() as *const ::libc::c_void) as *const u8,
                ::std::mem::size_of::<[libc::c_char; 8]>(),
            );
            let sb = core::slice::from_raw_parts(
                (c"sentinel".as_ptr().cast_mut() as *const ::libc::c_void) as *const u8,
                ::std::mem::size_of::<[libc::c_char; 8]>(),
            );
            let mut diff = 0_i32;
            for (x, y) in sa.iter().zip(sb.iter()) {
                if x != y {
                    diff = (*x as i32) - (*y as i32);
                    break;
                }
            }
            diff
        }) == (0))
    );
    assert!(((libc::fclose(fp)) == (0)));
    assert!(((libc::unlink(sibling)) == (0)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
