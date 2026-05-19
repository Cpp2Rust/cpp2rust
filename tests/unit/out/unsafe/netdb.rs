extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_getaddrinfo_0() {
    let mut hints: addrinfo = std::mem::zeroed::<addrinfo>();
    {
        let byte_0 =
            ((&mut hints as *mut addrinfo) as *mut addrinfo as *mut ::libc::c_void) as *mut u8;
        for offset in 0..::std::mem::size_of::<addrinfo>() as u64 {
            *byte_0.offset(offset as isize) = 0 as u8;
        }
        ((&mut hints as *mut addrinfo) as *mut addrinfo as *mut ::libc::c_void)
    };
    hints.ai_family = 2;
    hints.ai_socktype = libc::SOCK_STREAM;
    hints.ai_flags = 4;
    let mut res: *mut addrinfo = std::ptr::null_mut();
    assert!(
        ((((libc::getaddrinfo(
            (b"127.0.0.1\0".as_ptr().cast_mut()).cast_const() as *const i8,
            (b"80\0".as_ptr().cast_mut()).cast_const() as *const i8,
            (&mut hints as *mut addrinfo).cast_const(),
            (&mut res as *mut *mut addrinfo)
        )) == (0)) as i32)
            != 0)
    );
    assert!((((!((res).is_null())) as i32) != 0));
    assert!((((((*res).ai_family) == (2)) as i32) != 0));
    assert!((((((*res).ai_socktype) == (libc::SOCK_STREAM)) as i32) != 0));
    libc::freeaddrinfo(res);
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_getaddrinfo_0() });
    return 0;
}
