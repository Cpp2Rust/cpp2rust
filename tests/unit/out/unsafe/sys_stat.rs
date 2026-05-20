extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_stat_0() {
    let mut path: *const u8 = (b"/tmp/cpp2rust_stat_test.tmp\0".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut ::std::fs::File =
        match std::ffi::CStr::from_ptr((b"wb\0".as_ptr().cast_mut()).cast_const() as *const i8)
            .to_str()
            .expect("invalid c-string")
        {
            v if v == "rb" => std::fs::OpenOptions::new()
                .read(true)
                .open(
                    std::ffi::CStr::from_ptr(path as *const i8)
                        .to_str()
                        .expect("invalid c-string"),
                )
                .ok()
                .map_or(std::ptr::null_mut(), |f| Box::into_raw(Box::new(f))),
            v if v == "wb" => std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(
                    std::ffi::CStr::from_ptr(path as *const i8)
                        .to_str()
                        .expect("invalid c-string"),
                )
                .ok()
                .map_or(std::ptr::null_mut(), |f| Box::into_raw(Box::new(f))),
            _ => panic!("unsupported mode"),
        };
    assert!((((!((fp).is_null())) as i32) != 0));
    {
        let bytes =
            std::ffi::CStr::from_ptr((b"hello\0".as_ptr().cast_mut()).cast_const() as *const i8)
                .to_bytes();
        match (*fp).write_all(bytes) {
            Ok(()) => 0,
            Err(_) => -1,
        }
    };
    assert!(
        (((({
            Box::from_raw(fp);
            0
        }) == (0)) as i32)
            != 0)
    );
    let mut st: stat = std::mem::zeroed::<stat>();
    assert!(((((libc::stat(path as *const i8, (&mut st as *mut stat))) == (0)) as i32) != 0));
    assert!(((((st.st_size) == (5_i64)) as i32) != 0));
    libc::unlink(path as *const i8);
}
pub unsafe fn test_fstat_1() {
    let mut path: *const u8 = (b"/tmp/cpp2rust_fstat_test.tmp\0".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut ::std::fs::File =
        match std::ffi::CStr::from_ptr((b"wb\0".as_ptr().cast_mut()).cast_const() as *const i8)
            .to_str()
            .expect("invalid c-string")
        {
            v if v == "rb" => std::fs::OpenOptions::new()
                .read(true)
                .open(
                    std::ffi::CStr::from_ptr(path as *const i8)
                        .to_str()
                        .expect("invalid c-string"),
                )
                .ok()
                .map_or(std::ptr::null_mut(), |f| Box::into_raw(Box::new(f))),
            v if v == "wb" => std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(
                    std::ffi::CStr::from_ptr(path as *const i8)
                        .to_str()
                        .expect("invalid c-string"),
                )
                .ok()
                .map_or(std::ptr::null_mut(), |f| Box::into_raw(Box::new(f))),
            _ => panic!("unsupported mode"),
        };
    assert!((((!((fp).is_null())) as i32) != 0));
    {
        let bytes = std::ffi::CStr::from_ptr(
            (b"hello world\0".as_ptr().cast_mut()).cast_const() as *const i8
        )
        .to_bytes();
        match (*fp).write_all(bytes) {
            Ok(()) => 0,
            Err(_) => -1,
        }
    };
    if !(fp).is_null() {
        match (*fp).sync_all() {
            Ok(_) => 0,
            Err(_) => -1,
        }
    } else {
        ::std::io::stdout().flush().unwrap();
        ::std::io::stderr().flush().unwrap();
        0
    };
    let mut fd: i32 = if fp == libcc2rs::cin_unsafe() {
        0
    } else if fp == libcc2rs::cout_unsafe() {
        1
    } else if fp == libcc2rs::cerr_unsafe() {
        2
    } else {
        ::std::os::fd::AsRawFd::as_raw_fd(&*fp)
    };
    let mut st: stat = std::mem::zeroed::<stat>();
    assert!(((((libc::fstat(fd, (&mut st as *mut stat))) == (0)) as i32) != 0));
    assert!(((((st.st_size) == (11_i64)) as i32) != 0));
    assert!(
        (((({
            Box::from_raw(fp);
            0
        }) == (0)) as i32)
            != 0)
    );
    libc::unlink(path as *const i8);
}
pub unsafe fn test_mkdir_2() {
    let mut path: *const u8 = (b"/tmp/cpp2rust_mkdir_test_dir\0".as_ptr().cast_mut()).cast_const();
    libc::rmdir(path as *const i8);
    assert!(((((libc::mkdir(path as *const i8, 493_u32 as ::libc::mode_t)) == (0)) as i32) != 0));
    let mut st: stat = std::mem::zeroed::<stat>();
    assert!(((((libc::stat(path as *const i8, (&mut st as *mut stat))) == (0)) as i32) != 0));
    assert!((((((st.st_mode) & (61440_u32)) == ((16384) as u32)) as i32) != 0));
    (*libcc2rs::cpp2rust_errno()) = 0;
    assert!(
        ((((libc::mkdir(path as *const i8, 493_u32 as ::libc::mode_t)) == (-1_i32)) as i32) != 0)
    );
    assert!(((((*libcc2rs::cpp2rust_errno()) == (17)) as i32) != 0));
    assert!(((((libc::rmdir(path as *const i8)) == (0)) as i32) != 0));
    (*libcc2rs::cpp2rust_errno()) = 0;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_stat_0() });
    (unsafe { test_fstat_1() });
    (unsafe { test_mkdir_2() });
    return 0;
}
