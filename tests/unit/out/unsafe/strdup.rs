extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct record {
    pub name: *mut libc::c_char,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: *mut libc::c_char =
        libcc2rs::strdup_unsafe((c"hello".as_ptr().cast_mut()).cast_const());
    assert!((((!((d).is_null())) as i32) != 0));
    assert!(
        ((((libc::strcmp(
            (d).cast_const(),
            (c"hello".as_ptr().cast_mut()).cast_const()
        )) == (0)) as i32)
            != 0)
    );
    libcc2rs::free_unsafe((d as *mut libc::c_char as *mut ::libc::c_void));
    let mut p: *const libc::c_char = (c"world".as_ptr().cast_mut()).cast_const();
    let mut buf: [libc::c_char; 4] = [
        (('a' as i32) as libc::c_char),
        (('b' as i32) as libc::c_char),
        (('c' as i32) as libc::c_char),
        (('\0' as i32) as libc::c_char),
    ];
    let mut d2: *mut libc::c_char = libcc2rs::strdup_unsafe(p);
    assert!((((!((d2).is_null())) as i32) != 0));
    assert!(((((libc::strcmp((d2).cast_const(), p)) == (0)) as i32) != 0));
    libcc2rs::free_unsafe((d2 as *mut libc::c_char as *mut ::libc::c_void));
    let mut d3: *mut libc::c_char = libcc2rs::strdup_unsafe((buf.as_mut_ptr()).cast_const());
    assert!((((!((d3).is_null())) as i32) != 0));
    assert!(
        ((((libc::strcmp((d3).cast_const(), (buf.as_mut_ptr()).cast_const())) == (0)) as i32) != 0)
    );
    libcc2rs::free_unsafe((d3 as *mut libc::c_char as *mut ::libc::c_void));
    let mut d4: *mut libc::c_char = std::ptr::null_mut();
    d4 = libcc2rs::strdup_unsafe(p);
    assert!((((!((d4).is_null())) as i32) != 0));
    assert!(((((libc::strcmp((d4).cast_const(), p)) == (0)) as i32) != 0));
    libcc2rs::free_unsafe((d4 as *mut libc::c_char as *mut ::libc::c_void));
    let mut rec: record = record {
        name: std::ptr::null_mut(),
    };
    let mut r: *mut record = (&mut rec as *mut record);
    (*r).name = libcc2rs::strdup_unsafe(p);
    assert!((((!(((*r).name).is_null())) as i32) != 0));
    assert!(((((libc::strcmp(((*r).name).cast_const(), p)) == (0)) as i32) != 0));
    libcc2rs::free_unsafe(((*r).name as *mut libc::c_char as *mut ::libc::c_void));
    return 0;
}
