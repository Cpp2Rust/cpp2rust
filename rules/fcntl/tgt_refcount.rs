// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f1(a0: i32, a1: i32, va: &[VaArg]) -> i32 {
    let __res = match a1 {
        ::libc::F_GETFL => FdRegistry::with_fd(a0, |__fd| {
            nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_GETFL)
        }),
        ::libc::F_SETFL => {
            let __flags = nix::fcntl::OFlag::from_bits_retain(i32::get(&va[0]));
            FdRegistry::with_fd(a0, |__fd| {
                nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_SETFL(__flags))
            })
        }
        ::libc::F_GETFD => FdRegistry::with_fd(a0, |__fd| {
            nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_GETFD)
        }),
        ::libc::F_SETFD => {
            let __flags = nix::fcntl::FdFlag::from_bits_retain(i32::get(&va[0]));
            FdRegistry::with_fd(a0, |__fd| {
                nix::fcntl::fcntl(__fd, nix::fcntl::FcntlArg::F_SETFD(__flags))
            })
        }
        __cmd => panic!("fcntl: unsupported cmd {}", __cmd),
    };
    match __res {
        Ok(__r) => __r,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

fn f2(a0: Ptr<u8>, a1: i32, va: &[VaArg]) -> i32 {
    let __mode = match va.first() {
        Some(__m) => nix::sys::stat::Mode::from_bits_truncate(i32::get(__m) as ::libc::mode_t),
        None => nix::sys::stat::Mode::empty(),
    };
    match nix::fcntl::open(
        a0.to_rust_string().as_str(),
        nix::fcntl::OFlag::from_bits_retain(a1),
        __mode,
    ) {
        Ok(__ofd) => FdRegistry::register(__ofd),
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

fn f3() -> i32 {
    ::libc::O_CREAT
}

fn f4() -> i32 {
    ::libc::O_TRUNC
}

fn f5() -> i32 {
    ::libc::O_APPEND
}

fn f6() -> i32 {
    ::libc::O_EXCL
}

fn f7() -> i32 {
    ::libc::O_NONBLOCK
}

fn f8() -> i32 {
    ::libc::O_CLOEXEC
}

fn f9() -> i32 {
    ::libc::O_RDONLY
}

fn f10() -> i32 {
    ::libc::O_WRONLY
}

fn f11() -> i32 {
    ::libc::O_RDWR
}
