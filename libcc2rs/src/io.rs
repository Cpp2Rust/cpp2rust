// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::{AnyPtr, AsPointer, CFile, Ptr, Value};
use std::cell::{RefCell, UnsafeCell};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;

thread_local! {
    static SAFE_STDIN: Value<std::fs::File> = Rc::new(RefCell::new(std::fs::File::from(
        std::io::stdin().as_fd().try_clone_to_owned().unwrap(),
    )));
    static SAFE_STDOUT: Value<std::fs::File> = Rc::new(RefCell::new(std::fs::File::from(
        std::io::stdout().as_fd().try_clone_to_owned().unwrap(),
    )));
    static SAFE_STDERR: Value<std::fs::File> = Rc::new(RefCell::new(std::fs::File::from(
        std::io::stderr().as_fd().try_clone_to_owned().unwrap(),
    )));
    static UNSAFE_STDIN: UnsafeCell<std::fs::File> = unsafe {
        UnsafeCell::new(
            std::fs::File::from_raw_fd(
                std::io::stdin()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
        ))
    };
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

thread_local! {
    static C_STDIN: Value<CFile> = Rc::new(RefCell::new(CFile::new(0)));
    static C_STDOUT: Value<CFile> = Rc::new(RefCell::new(CFile::new(1)));
    static C_STDERR: Value<CFile> = Rc::new(RefCell::new(CFile::new(2)));
}

pub fn c_stdin() -> Ptr<CFile> {
    C_STDIN.with(AsPointer::as_pointer)
}

pub fn c_stdout() -> Ptr<CFile> {
    C_STDOUT.with(AsPointer::as_pointer)
}

pub fn c_stderr() -> Ptr<CFile> {
    C_STDERR.with(AsPointer::as_pointer)
}

pub fn cin() -> Ptr<std::fs::File> {
    SAFE_STDIN.with(AsPointer::as_pointer)
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
/// thread finishes.
pub unsafe fn cin_unsafe() -> *mut std::fs::File {
    UNSAFE_STDIN.with(UnsafeCell::get)
}

/// # Safety
///
/// The caller must ensure that the returned pointer is not used after the
/// thread finishes.
pub unsafe fn cout_unsafe() -> *mut std::fs::File {
    UNSAFE_STDOUT.with(UnsafeCell::get)
}

/// # Safety
///
/// The caller must ensure that the returned pointer is not used after the
/// thread finishes.
pub unsafe fn cerr_unsafe() -> *mut std::fs::File {
    UNSAFE_STDERR.with(UnsafeCell::get)
}

pub fn fread_refcount(a0: AnyPtr, a1: usize, a2: usize, a3: Ptr<CFile>) -> usize {
    let total = a1.saturating_mul(a2);
    if total == 0 {
        return 0;
    }
    let dst = a0.reinterpret_cast::<u8>();
    let read_bytes = dst.with_slice_mut(total, |buf| a3.with_mut(|f| f.read(buf)));
    read_bytes / a1
}

pub fn fwrite_refcount(a0: AnyPtr, a1: usize, a2: usize, a3: Ptr<CFile>) -> usize {
    let total = a1.saturating_mul(a2);
    if total == 0 {
        return 0;
    }
    let src = a0.reinterpret_cast::<u8>();
    let written = src.with_slice(total, |bytes| a3.with_mut(|f| f.write(bytes)));
    written / a1
}

unsafe extern "C" {
    #[cfg(target_os = "linux")]
    #[link_name = "stdin"]
    static mut LIBC_STDIN: *mut libc::FILE;
    #[cfg(target_os = "linux")]
    #[link_name = "stdout"]
    static mut LIBC_STDOUT: *mut libc::FILE;
    #[cfg(target_os = "linux")]
    #[link_name = "stderr"]
    static mut LIBC_STDERR: *mut libc::FILE;

    #[cfg(target_os = "macos")]
    #[link_name = "__stdinp"]
    static mut LIBC_STDIN: *mut libc::FILE;
    #[cfg(target_os = "macos")]
    #[link_name = "__stdoutp"]
    static mut LIBC_STDOUT: *mut libc::FILE;
    #[cfg(target_os = "macos")]
    #[link_name = "__stderrp"]
    static mut LIBC_STDERR: *mut libc::FILE;
}

/// # Safety
///
/// Returns the libc `stdin` handle. The pointer is valid for the process
/// lifetime.
pub unsafe fn stdin_unsafe() -> *mut libc::FILE {
    unsafe { LIBC_STDIN }
}

/// # Safety
///
/// Returns the libc `stdout` handle.
pub unsafe fn stdout_unsafe() -> *mut libc::FILE {
    unsafe { LIBC_STDOUT }
}

/// # Safety
///
/// Returns the libc `stderr` handle.
pub unsafe fn stderr_unsafe() -> *mut libc::FILE {
    unsafe { LIBC_STDERR }
}

/// # Safety
///
/// Same contract as C's `fwrite`.
pub unsafe fn fwrite_unsafe(
    a0: *const ::std::ffi::c_void,
    a1: usize,
    a2: usize,
    a3: *mut libc::FILE,
) -> usize {
    unsafe { libc::fwrite(a0, a1, a2, a3) }
}

/// # Safety
///
/// Same contract as C's `fread`.
pub unsafe fn fread_unsafe(
    a0: *mut ::std::ffi::c_void,
    a1: usize,
    a2: usize,
    a3: *mut libc::FILE,
) -> usize {
    unsafe { libc::fread(a0, a1, a2, a3) }
}
