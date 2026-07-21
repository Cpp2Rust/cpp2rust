// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::io::prelude::*;
use std::os::fd::AsFd;

fn t1() -> Ptr<::std::fs::File> {
    Ptr::null()
}

fn f1(a0: Ptr<u8>, a1: Ptr<u8>) -> Ptr<::std::fs::File> {
    match a1.to_rust_string() {
        v if v == "rb" => std::fs::OpenOptions::new()
            .read(true)
            .open(a0.to_rust_string())
            .ok()
            .map_or(Ptr::null(), |f| Ptr::alloc(f)),
        v if v == "wb" => std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(a0.to_rust_string())
            .ok()
            .map_or(Ptr::null(), |f| Ptr::alloc(f)),
        _ => panic!("unsupported mode"),
    }
}

fn f2(a0: Ptr<::std::fs::File>) -> i32 {
    a0.delete();
    0
}

fn f3(a0: Ptr<::std::fs::File>) -> i64 {
    a0.with_mut(|v| match v.stream_position() {
        Ok(pos) => pos as i64,
        Err(_) => -1,
    })
}

fn f4(a0: &mut ::std::fs::File, a1: i64, a2: i32) -> i32 {
    if (match a2 {
        0 => a0.seek(std::io::SeekFrom::Start(a1 as u64)),
        1 => a0.seek(std::io::SeekFrom::Current(a1)),
        2 => a0.seek(std::io::SeekFrom::End(a1)),
        _ => Err(std::io::Error::other("unsupported whence for fseek.")),
    })
    .is_ok()
    {
        0
    } else {
        -1
    }
}

fn f5(a0: AnyPtr, a1: usize, a2: usize, a3: Ptr<::std::fs::File>) -> usize {
    let __a0 = a0;
    let __a1 = a1;
    let __a2 = a2;
    let __a3 = a3.clone();
    libcc2rs::fread_refcount(__a0, __a1, __a2, __a3)
}

fn f6(a0: AnyPtr, a1: usize, a2: usize, a3: Ptr<::std::fs::File>) -> usize {
    let __a0 = a0;
    let __a1 = a1;
    let __a2 = a2;
    let __a3 = a3.clone();
    libcc2rs::fwrite_refcount(__a0, __a1, __a2, __a3)
}

fn f7(a0: Ptr<::std::fs::File>) -> i32 {
    if !a0.is_null() {
        match a0.with_mut(|v| v.sync_all()) {
            Ok(_) => 0,
            Err(_) => -1,
        }
    } else {
        ::std::io::stdout().flush().unwrap();
        ::std::io::stderr().flush().unwrap();
        0
    }
}

fn f8() -> Ptr<::std::fs::File> {
    libcc2rs::cout()
}

fn f9() -> Ptr<::std::fs::File> {
    libcc2rs::cerr()
}

fn f10() -> Ptr<::std::fs::File> {
    libcc2rs::cin()
}

fn f11(a0: i32, a1: Ptr<::std::fs::File>) -> i32 {
    let __c = a0 as u8;
    match a1.with_mut(|__f| ::std::io::Write::write_all(__f, &[__c])) {
        Ok(()) => __c as i32,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e.raw_os_error().unwrap_or(::libc::EIO));
            -1
        }
    }
}

fn f12(a0: Ptr<u8>, a1: Ptr<::std::fs::File>) -> i32 {
    let __bytes: Vec<u8> = a0.to_c_string_iterator().collect();
    match a1.with_mut(|__f| ::std::io::Write::write_all(__f, &__bytes)) {
        Ok(()) => 0,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e.raw_os_error().unwrap_or(::libc::EIO));
            -1
        }
    }
}

fn f13(a0: Ptr<u8>) -> i32 {
    let __bytes: Vec<u8> = a0.to_c_string_iterator().collect();
    match libcc2rs::cout().with_mut(|__f| {
        ::std::io::Write::write_all(__f, &__bytes)
            .and_then(|_| ::std::io::Write::write_all(__f, b"\n"))
    }) {
        Ok(()) => 0,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e.raw_os_error().unwrap_or(::libc::EIO));
            -1
        }
    }
}

fn f17(a0: Ptr<u8>, a1: i32, a2: Ptr<::std::fs::File>) -> Ptr<u8> {
    let __buf = a0.clone();
    let __n = a1;
    let __stream = a2.clone();
    if __n <= 0 {
        Ptr::null()
    } else {
        let __max = (__n - 1) as usize;
        let mut __dst = __buf.clone();
        let mut __count: usize = 0;
        let mut __failed = false;
        while __count < __max {
            let mut __b: [u8; 1] = [0];
            match __stream.with_mut(|__f| ::std::io::Read::read(__f, &mut __b)) {
                Ok(0) => break,
                Ok(_) => {
                    __dst.write(__b[0]);
                    __dst += 1;
                    __count += 1;
                    if __b[0] == b'\n' {
                        break;
                    }
                }
                Err(__e) => {
                    if __e.kind() != ::std::io::ErrorKind::Interrupted {
                        libcc2rs::cpp2rust_errno().write(__e.raw_os_error().unwrap_or(::libc::EIO));
                        __failed = true;
                        break;
                    }
                }
            }
        }
        if __failed || __count == 0 {
            Ptr::null()
        } else {
            __dst.write(0);
            __buf
        }
    }
}

fn f18(a0: Ptr<u8>, a1: Ptr<u8>, a2: Ptr<::std::fs::File>) -> Ptr<::std::fs::File> {
    let __stream = a2.clone();
    let __new = match a1.to_rust_string().as_str() {
        "rb" => ::std::fs::OpenOptions::new()
            .read(true)
            .open(a0.to_rust_string()),
        "wb" => ::std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(a0.to_rust_string()),
        _ => panic!("unsupported mode"),
    };
    match __new {
        Ok(__f) => {
            __stream.write(__f);
            __stream
        }
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e.raw_os_error().unwrap_or(::libc::EIO));
            Ptr::null()
        }
    }
}

fn f19(a0: Ptr<::std::fs::File>, a1: i64, a2: i32) -> i32 {
    let __r = a0.with_mut(|__f| match a2 {
        0 => ::std::io::Seek::seek(__f, ::std::io::SeekFrom::Start(a1 as u64)),
        1 => ::std::io::Seek::seek(__f, ::std::io::SeekFrom::Current(a1)),
        2 => ::std::io::Seek::seek(__f, ::std::io::SeekFrom::End(a1)),
        _ => Err(::std::io::Error::other("unsupported whence for fseeko.")),
    });
    match __r {
        Ok(_) => 0,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e.raw_os_error().unwrap_or(::libc::EINVAL));
            -1
        }
    }
}

fn f21(a0: Ptr<u8>, a1: usize, a2: Ptr<u8>, va: &[VaArg]) -> i32 {
    let __s = libcc2rs::format_c(&a2.to_rust_string(), va);
    let __b = __s.as_bytes();
    if a1 > 0 {
        let __n = ::std::cmp::min(__b.len(), a1 - 1);
        a0.with_slice_mut(__n + 1, |__dst| {
            __dst[..__n].copy_from_slice(&__b[..__n]);
            __dst[__n] = 0;
        });
    }
    __b.len() as i32
}

fn f22(a0: Ptr<u8>, a1: Ptr<u8>) -> i32 {
    match ::std::fs::rename(a0.to_rust_string(), a1.to_rust_string()) {
        Ok(()) => 0,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e.raw_os_error().unwrap_or(::libc::EIO));
            -1
        }
    }
}

fn f23(a0: Ptr<::std::fs::File>) -> i32 {
    let mut __b: [u8; 1] = [0];
    match a0.with_mut(|__f| ::std::io::Read::read(__f, &mut __b)) {
        Ok(0) => -1,
        Ok(_) => __b[0] as i32,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e.raw_os_error().unwrap_or(::libc::EIO));
            -1
        }
    }
}

fn f24(a0: Ptr<::std::fs::File>, a1: Ptr<u8>, a2: i32, a3: usize) -> i32 {
    /* std::fs::File is unbuffered */
    0
}
