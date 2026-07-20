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
            Sockaddr::encode(&__ss, &a1, &a2);
            0
        }
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

fn f13(a0: i32, a1: Ptr<Sockaddr>, a2: u32) -> i32 {
    match Sockaddr::decode(&a1, a2) {
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
            Sockaddr::encode(&__ss, &a1, &a2);
            0
        }
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

fn f16(a0: i32, a1: Ptr<Sockaddr>, a2: u32) -> i32 {
    match Sockaddr::decode(&a1, a2) {
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
                Some(__ss) => Sockaddr::encode(&__ss, &a4, &a5),
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
    match Sockaddr::decode(&a4, a5) {
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

fn f6(a0: i32, a1: i32, a2: i32) -> i32 {
    let __family = match a0 {
        ::libc::AF_INET => nix::sys::socket::AddressFamily::Inet,
        ::libc::AF_INET6 => nix::sys::socket::AddressFamily::Inet6,
        ::libc::AF_UNIX => nix::sys::socket::AddressFamily::Unix,
        __d => panic!("socket: unsupported domain {__d}"),
    };
    let __flags = nix::sys::socket::SockFlag::from_bits_truncate(a1);
    let __ty = match a1 & !nix::sys::socket::SockFlag::all().bits() {
        ::libc::SOCK_STREAM => nix::sys::socket::SockType::Stream,
        ::libc::SOCK_DGRAM => nix::sys::socket::SockType::Datagram,
        __t => panic!("socket: unsupported type {__t}"),
    };
    let __proto = match a2 {
        0 => None,
        ::libc::IPPROTO_TCP => Some(nix::sys::socket::SockProtocol::Tcp),
        ::libc::IPPROTO_UDP => Some(nix::sys::socket::SockProtocol::Udp),
        __p => panic!("socket: unsupported protocol {__p}"),
    };
    match nix::sys::socket::socket(__family, __ty, __flags, __proto) {
        Ok(__ofd) => FdRegistry::register(__ofd),
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

fn f11(a0: i32, a1: i32, a2: i32, a3: Ptr<i32>) -> i32 {
    let __family = match a0 {
        ::libc::AF_INET => nix::sys::socket::AddressFamily::Inet,
        ::libc::AF_INET6 => nix::sys::socket::AddressFamily::Inet6,
        ::libc::AF_UNIX => nix::sys::socket::AddressFamily::Unix,
        __d => panic!("socketpair: unsupported domain {__d}"),
    };
    let __flags = nix::sys::socket::SockFlag::from_bits_truncate(a1);
    let __ty = match a1 & !nix::sys::socket::SockFlag::all().bits() {
        ::libc::SOCK_STREAM => nix::sys::socket::SockType::Stream,
        ::libc::SOCK_DGRAM => nix::sys::socket::SockType::Datagram,
        __t => panic!("socketpair: unsupported type {__t}"),
    };
    let __proto = match a2 {
        0 => None,
        ::libc::IPPROTO_TCP => Some(nix::sys::socket::SockProtocol::Tcp),
        ::libc::IPPROTO_UDP => Some(nix::sys::socket::SockProtocol::Udp),
        __p => panic!("socketpair: unsupported protocol {__p}"),
    };
    match nix::sys::socket::socketpair(__family, __ty, __proto, __flags) {
        Ok((__a, __b)) => {
            let __sv = a3.clone();
            __sv.write(FdRegistry::register(__a));
            __sv.offset(1).write(FdRegistry::register(__b));
            0
        }
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

fn f15(a0: i32, a1: Ptr<Sockaddr>, a2: Ptr<u32>, a3: i32) -> i32 {
    let __flags = nix::sys::socket::SockFlag::from_bits_truncate(a3);
    let __raw = FdRegistry::with_fd(a0, |__fd| {
        nix::sys::socket::accept4(std::os::fd::AsRawFd::as_raw_fd(&__fd), __flags)
    });
    match __raw {
        Ok(__new) => {
            let __addr = a1.clone();
            let __len = a2.clone();
            if !__addr.is_null() {
                match nix::sys::socket::getpeername::<nix::sys::socket::SockaddrStorage>(__new) {
                    Ok(__ss) => Sockaddr::encode(&__ss, &__addr, &__len),
                    Err(__e) => panic!("accept4: getpeername failed: {__e}"),
                }
            }
            /* nix 0.30 accept4 returns RawFd; no safe OwnedFd constructor exists */
            FdRegistry::register(unsafe {
                <std::os::fd::OwnedFd as std::os::fd::FromRawFd>::from_raw_fd(__new)
            })
        }
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}
