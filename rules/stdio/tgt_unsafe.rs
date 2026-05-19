// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::io::prelude::*;

fn types() -> Result<(), Box<dyn std::error::Error>> {
    let t1: *mut ::std::fs::File = std::ptr::null_mut();
    Ok(())
}

unsafe fn f1(a0: *const i8, a1: *const i8) -> *mut ::std::fs::File {
    match std::ffi::CStr::from_ptr(a1 as *const i8)
        .to_str()
        .expect("invalid c-string")
    {
        v if v == "rb" => std::fs::OpenOptions::new()
            .read(true)
            .open(
                std::ffi::CStr::from_ptr(a0 as *const i8)
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
                std::ffi::CStr::from_ptr(a0 as *const i8)
                    .to_str()
                    .expect("invalid c-string"),
            )
            .ok()
            .map_or(std::ptr::null_mut(), |f| Box::into_raw(Box::new(f))),
        _ => panic!("unsupported mode"),
    }
}

unsafe fn f2(a0: *mut ::std::fs::File) -> i32 {
    Box::from_raw(a0);
    0
}

unsafe fn f3(a0: *mut ::std::fs::File) -> i64 {
    match (*a0).stream_position() {
        Ok(pos) => pos as i64,
        Err(_) => -1,
    }
}

unsafe fn f4(a0: *mut ::std::fs::File, a1: i64, a2: i32) -> i32 {
    if (match a2 {
        0 => (*a0).seek(std::io::SeekFrom::Start(a1 as u64)),
        1 => (*a0).seek(std::io::SeekFrom::Current(a1)),
        2 => (*a0).seek(std::io::SeekFrom::End(a1)),
        _ => Err(std::io::Error::other("unsupported whence for fseek.")),
    })
    .is_ok()
    {
        0
    } else {
        -1
    }
}

unsafe fn f5(a0: *mut ::libc::c_void, a1: u64, a2: u64, a3: *mut ::std::fs::File) -> u64 {
    let __a0 = a0 as *mut ::std::ffi::c_void;
    let __a1 = a1;
    let __a2 = a2;
    let __a3 = a3;
    libcc2rs::fread_unsafe(__a0, __a1, __a2, __a3)
}

unsafe fn f6(a0: *const ::libc::c_void, a1: u64, a2: u64, a3: *mut ::std::fs::File) -> u64 {
    let __a0 = a0 as *const ::std::ffi::c_void;
    let __a1 = a1;
    let __a2 = a2;
    let __a3 = a3;
    libcc2rs::fwrite_unsafe(__a0, __a1, __a2, __a3)
}

unsafe fn f7(a0: *mut ::std::fs::File) -> i32 {
    if !(a0).is_null() {
        match (*a0).sync_all() {
            Ok(_) => 0,
            Err(_) => -1,
        }
    } else {
        ::std::io::stdout().flush().unwrap();
        ::std::io::stderr().flush().unwrap();
        0
    }
}

unsafe fn f8() -> *mut ::std::fs::File {
    libcc2rs::cout_unsafe()
}

unsafe fn f9() -> *mut ::std::fs::File {
    libcc2rs::cerr_unsafe()
}

unsafe fn f10() -> *mut ::std::fs::File {
    libcc2rs::cin_unsafe()
}

unsafe fn f11(a0: i32, a1: *mut ::std::fs::File) -> i32 {
    match (*a1).write_all(&[a0 as u8]) {
        Ok(()) => a0 & 0xff,
        Err(_) => -1,
    }
}

unsafe fn f12(a0: *const u8, a1: *mut ::std::fs::File) -> i32 {
    let bytes = std::ffi::CStr::from_ptr(a0 as *const i8).to_bytes();
    match (*a1).write_all(bytes) {
        Ok(()) => 0,
        Err(_) => -1,
    }
}
