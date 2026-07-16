// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t1() -> libcc2rs::Sockaddr {
    Default::default()
}

fn t2() -> libcc2rs::SockaddrStorage {
    Default::default()
}

fn t3() -> libcc2rs::SockaddrUn {
    Default::default()
}

#[cfg(target_os = "linux")]
fn f11(a0: i32, a1: i32, a2: i32, a3: Ptr<i32>) -> i32 {
    use nix::sys::socket::{AddressFamily, SockFlag, SockProtocol, SockType};
    let __out = a3.clone();
    let __flag_bits = ::libc::SOCK_CLOEXEC | ::libc::SOCK_NONBLOCK;
    match (
        AddressFamily::from_i32(a0),
        SockType::try_from(a1 & !__flag_bits),
    ) {
        (Some(__f), Ok(__t)) => {
            let __flags = SockFlag::from_bits_truncate(a1 & __flag_bits);
            let __proto: Option<SockProtocol> = None;
            match nix::sys::socket::socketpair(__f, __t, __proto, __flags) {
                Ok((__a, __b)) => {
                    __out.write(::std::os::fd::IntoRawFd::into_raw_fd(__a));
                    __out
                        .offset(1)
                        .write(::std::os::fd::IntoRawFd::into_raw_fd(__b));
                    0
                }
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        }
        _ => {
            libcc2rs::cpp2rust_errno().write(::libc::EINVAL);
            -1
        }
    }
}

#[cfg(target_os = "macos")]
fn f11(a0: i32, a1: i32, a2: i32, a3: Ptr<i32>) -> i32 {
    use nix::sys::socket::{AddressFamily, SockFlag, SockProtocol, SockType};
    let __out = a3.clone();
    match (AddressFamily::from_i32(a0), SockType::try_from(a1)) {
        (Some(__f), Ok(__t)) => {
            let __proto: Option<SockProtocol> = None;
            match nix::sys::socket::socketpair(__f, __t, __proto, SockFlag::empty()) {
                Ok((__a, __b)) => {
                    __out.write(::std::os::fd::IntoRawFd::into_raw_fd(__a));
                    __out
                        .offset(1)
                        .write(::std::os::fd::IntoRawFd::into_raw_fd(__b));
                    0
                }
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        }
        _ => {
            libcc2rs::cpp2rust_errno().write(::libc::EINVAL);
            -1
        }
    }
}

fn f9(a0: i32, a1: AnyPtr, a2: usize, a3: i32) -> isize {
    let __buf = a1.reinterpret_cast::<u8>();
    match __buf.with_slice_mut(a2, |__s| {
        nix::sys::socket::recv(a0, __s, nix::sys::socket::MsgFlags::from_bits_truncate(a3))
    }) {
        Ok(__n) => __n as isize,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

fn f10(a0: i32, a1: AnyPtr, a2: usize, a3: i32) -> isize {
    let __buf = a1.reinterpret_cast::<u8>();
    match __buf.with_slice(a2, |__s| {
        nix::sys::socket::send(a0, __s, nix::sys::socket::MsgFlags::from_bits_truncate(a3))
    }) {
        Ok(__n) => __n as isize,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}
