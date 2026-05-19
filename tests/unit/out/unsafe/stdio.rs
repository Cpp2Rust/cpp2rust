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
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_fputc_0() });
    (unsafe { test_fputs_1() });
    return 0;
}
