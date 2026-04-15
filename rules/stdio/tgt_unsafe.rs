// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::io::prelude::*;

fn types() -> Result<(), Box<dyn std::error::Error>> {
    let t1: *mut ::std::fs::File = Default::default();
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

pub unsafe fn f5(
    a0: *mut ::libc::c_void,
    a1: u64,
    a2: u64,
    a3: *mut ::std::fs::File,
) -> u64 {
    let total = a1.saturating_mul(a2) as usize;
    let mut dst = a0 as *mut u8;

    let mut f = (*a3).try_clone().expect("try_clone failed");
    let mut reader = std::io::BufReader::with_capacity(64 * 1024, f);

    let mut read_bytes: usize = 0;
    let mut buffer: [u8; 8192] = [0; 8192];

    while read_bytes < total {
        let remaining = total - read_bytes;
        let to_read = std::cmp::min(buffer.len(), remaining);

        let n = match std::io::Read::read(&mut reader, &mut buffer[..to_read]) {
            Ok(0) => break,
            Ok(n) => n,
            Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(e) => panic!("Unhandled error in fread: {e}"),
        };

        for i in 0..n {
            *dst = buffer[i];
            dst = dst.offset(1);
        }

        read_bytes += n;
    }

    (read_bytes / a1 as usize) as u64
}

pub unsafe fn f6(
    a0: *const ::libc::c_void,
    a1: u64,
    a2: u64,
    a3: *mut ::std::fs::File,
) -> u64 {
    let total = a1.saturating_mul(a2) as usize;
    let mut src = a0 as *mut u8;

    let mut f = (*a3).try_clone().expect("try_clone failed");
    let mut writer = std::io::BufWriter::with_capacity(64 * 1024, f);

    let mut written_bytes: usize = 0;
    let mut buffer: [u8; 8192] = [0; 8192];

    while written_bytes < total {
        let remaining = total - written_bytes;
        let to_fill = std::cmp::min(buffer.len(), remaining);

        for i in 0..to_fill {
            buffer[i] = *src;
            src = src.offset(1);
        }

        let mut off = 0;
        while off < to_fill {
            match std::io::Write::write(&mut writer, &buffer[off..to_fill]) {
                Ok(0) => break,
                Ok(n) => off += n,
                Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(e) => panic!("Unhandled error in fwrite: {e}"),
            }
        }

        if off == 0 {
            break;
        }

        written_bytes += off;
    }

    std::io::Write::flush(&mut writer).expect("flush failed");

    (written_bytes / a1 as usize) as u64
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
