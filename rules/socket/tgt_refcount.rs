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

fn f12(a0: i32, a1: Ptr<Sockaddr>, a2: Ptr<u32>) -> i32 {
    match nix::sys::socket::getsockname::<nix::sys::socket::SockaddrStorage>(a0) {
        Ok(__ss) => {
            libcc2rs::encode_sockaddr(&__ss, &a1, &a2);
            0
        }
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

fn f13(a0: i32, a1: Ptr<Sockaddr>, a2: u32) -> i32 {
    match libcc2rs::decode_sockaddr(&a1, a2) {
        Some(__addr) => match nix::sys::socket::connect(a0, &*__addr) {
            Ok(()) => 0,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        },
        None => {
            libcc2rs::cpp2rust_errno().write(::libc::EAFNOSUPPORT);
            -1
        }
    }
}

fn f14(a0: i32, a1: Ptr<Sockaddr>, a2: Ptr<u32>) -> i32 {
    match nix::sys::socket::getpeername::<nix::sys::socket::SockaddrStorage>(a0) {
        Ok(__ss) => {
            libcc2rs::encode_sockaddr(&__ss, &a1, &a2);
            0
        }
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

fn f16(a0: i32, a1: Ptr<Sockaddr>, a2: u32) -> i32 {
    match libcc2rs::decode_sockaddr(&a1, a2) {
        Some(__addr) => match nix::sys::socket::bind(a0, &*__addr) {
            Ok(()) => 0,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        },
        None => {
            libcc2rs::cpp2rust_errno().write(::libc::EAFNOSUPPORT);
            -1
        }
    }
}

fn f18(a0: i32, a1: AnyPtr, a2: usize, a3: i32, a4: Ptr<Sockaddr>, a5: Ptr<u32>) -> isize {
    let __buf = a1.reinterpret_cast::<u8>();
    match __buf.with_slice_mut(a2, |__s| {
        nix::sys::socket::recvfrom::<nix::sys::socket::SockaddrStorage>(a0, __s)
    }) {
        Ok((__n, __from)) => {
            match __from {
                Some(__ss) => libcc2rs::encode_sockaddr(&__ss, &a4, &a5),
                None => {}
            }
            __n as isize
        }
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

fn f19(a0: i32, a1: AnyPtr, a2: usize, a3: i32, a4: Ptr<Sockaddr>, a5: u32) -> isize {
    let __buf = a1.reinterpret_cast::<u8>();
    match libcc2rs::decode_sockaddr(&a4, a5) {
        Some(__addr) => match __buf.with_slice(a2, |__s| {
            nix::sys::socket::sendto(
                a0,
                __s,
                &*__addr,
                nix::sys::socket::MsgFlags::from_bits_truncate(a3),
            )
        }) {
            Ok(__n) => __n as isize,
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        },
        None => {
            libcc2rs::cpp2rust_errno().write(::libc::EAFNOSUPPORT);
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
