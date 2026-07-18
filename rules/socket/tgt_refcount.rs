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
