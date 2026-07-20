// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

#[cfg(target_os = "linux")]
fn f1() -> i32 {
    ::libc::EFD_CLOEXEC
}

#[cfg(target_os = "linux")]
fn f2() -> i32 {
    ::libc::EFD_NONBLOCK
}

#[cfg(target_os = "linux")]
fn f3() -> i32 {
    ::libc::EFD_SEMAPHORE
}

#[cfg(target_os = "linux")]
fn f4(a0: u32, a1: i32) -> i32 {
    match nix::sys::eventfd::EventFd::from_value_and_flags(
        a0,
        nix::sys::eventfd::EfdFlags::from_bits_truncate(a1),
    ) {
        Ok(__efd) => FdRegistry::register(std::os::fd::OwnedFd::from(__efd)),
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}
