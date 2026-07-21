// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f1(a0: i32) -> i32 {
    FdRegistry::close(a0)
}

fn f3(a0: i32, a1: AnyPtr, a2: usize) -> isize {
    match FdRegistry::with_fd(a0, |__fd| {
        a1.reinterpret_cast::<u8>()
            .with_slice_mut(a2, |__buf| nix::unistd::read(__fd, __buf))
    }) {
        Ok(__n) => __n as isize,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

fn f4(a0: Ptr<u8>) -> i32 {
    match nix::unistd::unlink(a0.to_rust_string().as_str()) {
        Ok(()) => 0,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

fn f8() -> u32 {
    nix::unistd::geteuid().as_raw()
}

fn f9(a0: Ptr<u8>, a1: usize) -> i32 {
    match nix::unistd::gethostname() {
        Ok(__name) => {
            let __bytes = __name.as_encoded_bytes();
            let __n = __bytes.len().min(a1.saturating_sub(1));
            if a1 > 0 {
                a0.with_slice_mut(__n + 1, |__s| {
                    __s[..__n].copy_from_slice(&__bytes[..__n]);
                    __s[__n] = 0;
                });
            }
            0
        }
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

fn f10(a0: i32, a1: AnyPtr, a2: usize) -> isize {
    match FdRegistry::with_fd(a0, |__fd| {
        a1.reinterpret_cast::<u8>()
            .with_slice(a2, |__buf| nix::unistd::write(__fd, __buf))
    }) {
        Ok(__n) => __n as isize,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

fn f11(a0: Ptr<u8>) -> i32 {
    match ::std::fs::remove_dir(a0.to_rust_string()) {
        Ok(()) => 0,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e.raw_os_error().unwrap_or(::libc::EIO));
            -1
        }
    }
}
