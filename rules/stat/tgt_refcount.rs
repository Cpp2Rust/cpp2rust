// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t1() -> libcc2rs::Stat {
    Default::default()
}

fn f1(a0: Ptr<u8>, a1: Ptr<Stat>) -> i32 {
    match nix::sys::stat::stat(a0.to_rust_string().as_str()) {
        Ok(__s) => {
            a1.with_mut(|__st| *__st = Stat::from_libc(&__s));
            0
        }
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

fn f2(a0: i32, a1: Ptr<Stat>) -> i32 {
    match FdRegistry::with_fd(a0, |__fd| nix::sys::stat::fstat(__fd)) {
        Ok(__s) => {
            a1.with_mut(|__st| *__st = Stat::from_libc(&__s));
            0
        }
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

fn f3(a0: Ptr<u8>, a1: ::libc::mode_t) -> i32 {
    match nix::unistd::mkdir(
        a0.to_rust_string().as_str(),
        nix::sys::stat::Mode::from_bits_truncate(a1),
    ) {
        Ok(()) => 0,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}
