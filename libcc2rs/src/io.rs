// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::{AnyPtr, AsPointer, Ptr, Value};
use std::cell::{RefCell, UnsafeCell};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;

thread_local! {
    static SAFE_STDOUT: Value<std::fs::File> = Rc::new(RefCell::new(std::fs::File::from(
        std::io::stdout().as_fd().try_clone_to_owned().unwrap(),
    )));
    static SAFE_STDERR: Value<std::fs::File> = Rc::new(RefCell::new(std::fs::File::from(
        std::io::stderr().as_fd().try_clone_to_owned().unwrap(),
    )));
    static UNSAFE_STDOUT: UnsafeCell<std::fs::File> = unsafe {
        UnsafeCell::new(
            std::fs::File::from_raw_fd(
                std::io::stdout()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
        ))
    };
    static UNSAFE_STDERR: UnsafeCell<std::fs::File> = unsafe {
        UnsafeCell::new(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
        ))
    };
}

pub fn cout() -> Ptr<std::fs::File> {
    SAFE_STDOUT.with(AsPointer::as_pointer)
}

pub fn cerr() -> Ptr<std::fs::File> {
    SAFE_STDERR.with(AsPointer::as_pointer)
}

/// # Safety
///
/// The caller must ensure that the returned pointer is not used after the
//  thread finishes.
pub unsafe fn cout_unsafe() -> *mut std::fs::File {
    UNSAFE_STDOUT.with(UnsafeCell::get)
}

/// # Safety
///
/// The caller must ensure that the returned pointer is not used after the
//  thread finishes.
pub unsafe fn cerr_unsafe() -> *mut std::fs::File {
    UNSAFE_STDERR.with(UnsafeCell::get)
}

pub fn fread_refcount(a0: AnyPtr, a1: u64, a2: u64, a3: Ptr<::std::fs::File>) -> u64 {
    let total = a1.saturating_mul(a2) as usize;
    let mut dst = a0
        .cast::<u8>()
        .expect("fread: only supporting u8 pointers")
        .clone();

    let f = (*a3.upgrade().deref())
        .try_clone()
        .expect("try_clone failed");
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

        for &byte in &buffer[..n] {
            dst.write(byte);
            dst = dst.offset(1);
        }

        read_bytes += n;
    }

    (read_bytes / a1 as usize) as u64
}

/// # Safety
///
/// `a0` must point to a writable buffer of at least `a1 * a2` bytes, and `a3`
/// must point to a valid, open `std::fs::File`.
pub unsafe fn fread_unsafe(
    a0: *mut ::std::ffi::c_void,
    a1: u64,
    a2: u64,
    a3: *mut ::std::fs::File,
) -> u64 {
    let total = a1.saturating_mul(a2) as usize;
    let mut dst = a0 as *mut u8;

    let f = unsafe { (*a3).try_clone().expect("try_clone failed") };
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

        for &byte in &buffer[..n] {
            unsafe {
                *dst = byte;
                dst = dst.offset(1);
            }
        }

        read_bytes += n;
    }

    (read_bytes / a1 as usize) as u64
}
