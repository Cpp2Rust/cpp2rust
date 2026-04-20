// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::{AsPointer, Ptr, Value};
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
