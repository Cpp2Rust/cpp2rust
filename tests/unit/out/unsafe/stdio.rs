extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_fputc_0() {
    match (*libcc2rs::cout_unsafe()).write_all(&[('H' as i32) as u8]) {
        Ok(()) => ('H' as i32) & 0xff,
        Err(_) => -1,
    };
    match (*libcc2rs::cout_unsafe()).write_all(&[('i' as i32) as u8]) {
        Ok(()) => ('i' as i32) & 0xff,
        Err(_) => -1,
    };
    match (*libcc2rs::cout_unsafe()).write_all(&[('\n' as i32) as u8]) {
        Ok(()) => ('\n' as i32) & 0xff,
        Err(_) => -1,
    };
}
pub unsafe fn test_fputs_1() {
    {
        let bytes =
            std::ffi::CStr::from_ptr((b"hello\0".as_ptr().cast_mut()).cast_const() as *const i8)
                .to_bytes();
        match (*libcc2rs::cout_unsafe()).write_all(bytes) {
            Ok(()) => 0,
            Err(_) => -1,
        }
    };
    match (*libcc2rs::cout_unsafe()).write_all(&[('\n' as i32) as u8]) {
        Ok(()) => ('\n' as i32) & 0xff,
        Err(_) => -1,
    };
    let mut s: *const u8 = (b"from variable\0".as_ptr().cast_mut()).cast_const();
    {
        let bytes = std::ffi::CStr::from_ptr(s as *const i8).to_bytes();
        match (*libcc2rs::cout_unsafe()).write_all(bytes) {
            Ok(()) => 0,
            Err(_) => -1,
        }
    };
    match (*libcc2rs::cout_unsafe()).write_all(&[('\n' as i32) as u8]) {
        Ok(()) => ('\n' as i32) & 0xff,
        Err(_) => -1,
    };
    let mut buf: [u8; 4] = [
        (('b' as i32) as u8),
        (('u' as i32) as u8),
        (('f' as i32) as u8),
        (('\0' as i32) as u8),
    ];
    {
        let bytes =
            std::ffi::CStr::from_ptr((buf.as_mut_ptr()).cast_const() as *const i8).to_bytes();
        match (*libcc2rs::cout_unsafe()).write_all(bytes) {
            Ok(()) => 0,
            Err(_) => -1,
        }
    };
    match (*libcc2rs::cout_unsafe()).write_all(&[('\n' as i32) as u8]) {
        Ok(()) => ('\n' as i32) & 0xff,
        Err(_) => -1,
    };
}
pub unsafe fn test_puts_2() {
    {
        let bytes = std::ffi::CStr::from_ptr(
            (b"puts hello\0".as_ptr().cast_mut()).cast_const() as *const i8
        )
        .to_bytes();
        let stdout = libcc2rs::cout_unsafe();
        let r1 = (*stdout).write_all(bytes);
        let r2 = (*stdout).write_all(b"\n");
        if r1.is_ok() && r2.is_ok() {
            0
        } else {
            -1
        }
    };
    let mut s: *const u8 = (b"puts variable\0".as_ptr().cast_mut()).cast_const();
    {
        let bytes = std::ffi::CStr::from_ptr(s as *const i8).to_bytes();
        let stdout = libcc2rs::cout_unsafe();
        let r1 = (*stdout).write_all(bytes);
        let r2 = (*stdout).write_all(b"\n");
        if r1.is_ok() && r2.is_ok() {
            0
        } else {
            -1
        }
    };
}
pub unsafe fn test_fileno_3() {
    assert!(
        ((((if libcc2rs::cin_unsafe() == libcc2rs::cin_unsafe() {
            0
        } else if libcc2rs::cin_unsafe() == libcc2rs::cout_unsafe() {
            1
        } else if libcc2rs::cin_unsafe() == libcc2rs::cerr_unsafe() {
            2
        } else {
            ::std::os::fd::AsRawFd::as_raw_fd(&*libcc2rs::cin_unsafe())
        }) == (0)) as i32)
            != 0)
    );
    assert!(
        ((((if libcc2rs::cout_unsafe() == libcc2rs::cin_unsafe() {
            0
        } else if libcc2rs::cout_unsafe() == libcc2rs::cout_unsafe() {
            1
        } else if libcc2rs::cout_unsafe() == libcc2rs::cerr_unsafe() {
            2
        } else {
            ::std::os::fd::AsRawFd::as_raw_fd(&*libcc2rs::cout_unsafe())
        }) == (1)) as i32)
            != 0)
    );
    assert!(
        ((((if libcc2rs::cerr_unsafe() == libcc2rs::cin_unsafe() {
            0
        } else if libcc2rs::cerr_unsafe() == libcc2rs::cout_unsafe() {
            1
        } else if libcc2rs::cerr_unsafe() == libcc2rs::cerr_unsafe() {
            2
        } else {
            ::std::os::fd::AsRawFd::as_raw_fd(&*libcc2rs::cerr_unsafe())
        }) == (2)) as i32)
            != 0)
    );
    let mut file: *const u8 = (b"/tmp/cpp2rust_fileno_test.tmp\0".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut ::std::fs::File =
        match std::ffi::CStr::from_ptr((b"wb\0".as_ptr().cast_mut()).cast_const() as *const i8)
            .to_str()
            .expect("invalid c-string")
        {
            v if v == "rb" => std::fs::OpenOptions::new()
                .read(true)
                .open(
                    std::ffi::CStr::from_ptr(file as *const i8)
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
                    std::ffi::CStr::from_ptr(file as *const i8)
                        .to_str()
                        .expect("invalid c-string"),
                )
                .ok()
                .map_or(std::ptr::null_mut(), |f| Box::into_raw(Box::new(f))),
            _ => panic!("unsupported mode"),
        };
    assert!((((!((fp).is_null())) as i32) != 0));
    assert!(
        ((((if fp == libcc2rs::cin_unsafe() {
            0
        } else if fp == libcc2rs::cout_unsafe() {
            1
        } else if fp == libcc2rs::cerr_unsafe() {
            2
        } else {
            ::std::os::fd::AsRawFd::as_raw_fd(&*fp)
        }) > (2)) as i32)
            != 0)
    );
    {
        Box::from_raw(fp);
        0
    };
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_fputc_0() });
    (unsafe { test_fputs_1() });
    (unsafe { test_puts_2() });
    (unsafe { test_fileno_3() });
    return 0;
}
