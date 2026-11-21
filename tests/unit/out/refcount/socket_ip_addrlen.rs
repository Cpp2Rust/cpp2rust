extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn check_name_0(mut addr: Ptr<libcc2rs::Sockaddr>, mut expected: libc::socklen_t) {
    let mut receiver: i32 = {
        let __family = match (addr.with(|__s| __s.sa_family) as i32) {
            ::libc::AF_INET => nix::sys::socket::AddressFamily::Inet,
            ::libc::AF_INET6 => nix::sys::socket::AddressFamily::Inet6,
            ::libc::AF_UNIX => nix::sys::socket::AddressFamily::Unix,
            __d => panic!("socket: unsupported domain {__d}"),
        };
        let __flags = nix::sys::socket::SockFlag::from_bits_truncate(libc::SOCK_DGRAM);
        let __ty = match libc::SOCK_DGRAM & !nix::sys::socket::SockFlag::all().bits() {
            ::libc::SOCK_STREAM => nix::sys::socket::SockType::Stream,
            ::libc::SOCK_DGRAM => nix::sys::socket::SockType::Datagram,
            __t => panic!("socket: unsupported type {__t}"),
        };
        let __proto = match 0 {
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
    };
    let mut sender: i32 = {
        let __family = match (addr.with(|__s| __s.sa_family) as i32) {
            ::libc::AF_INET => nix::sys::socket::AddressFamily::Inet,
            ::libc::AF_INET6 => nix::sys::socket::AddressFamily::Inet6,
            ::libc::AF_UNIX => nix::sys::socket::AddressFamily::Unix,
            __d => panic!("socket: unsupported domain {__d}"),
        };
        let __flags = nix::sys::socket::SockFlag::from_bits_truncate(libc::SOCK_DGRAM);
        let __ty = match libc::SOCK_DGRAM & !nix::sys::socket::SockFlag::all().bits() {
            ::libc::SOCK_STREAM => nix::sys::socket::SockType::Stream,
            ::libc::SOCK_DGRAM => nix::sys::socket::SockType::Datagram,
            __t => panic!("socket: unsupported type {__t}"),
        };
        let __proto = match 0 {
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
    };
    assert!((receiver >= 0));
    assert!((sender >= 0));
    let local: Value<libcc2rs::SockaddrStorage> = Rc::new(RefCell::new(Default::default()));
    let len: Value<libc::socklen_t> = Rc::new(RefCell::new((128usize as libc::socklen_t)));
    assert!(
        ({
            let mut __tmp_0 = { (local.as_pointer()).reinterpret_cast::<libcc2rs::Sockaddr>() };
            let mut __tmp_1 = { (len.as_pointer()) };
            match nix::sys::socket::getsockname::<nix::sys::socket::SockaddrStorage>(receiver) {
                Ok(__ss) => {
                    Sockaddr::encode(&__ss, &__tmp_0, &__tmp_1);
                    0
                }
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 0)
    );
    assert!(({ ((*len.borrow()) == expected) }));
    (*len.borrow_mut()) = (0 as libc::socklen_t);
    assert!(
        ({
            let mut __tmp_2 = { (local.as_pointer()).reinterpret_cast::<libcc2rs::Sockaddr>() };
            let mut __tmp_3 = { (len.as_pointer()) };
            match nix::sys::socket::getsockname::<nix::sys::socket::SockaddrStorage>(receiver) {
                Ok(__ss) => {
                    Sockaddr::encode(&__ss, &__tmp_2, &__tmp_3);
                    0
                }
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 0)
    );
    assert!(({ ((*len.borrow()) == expected) }));
    assert!(
        (match Sockaddr::decode(&addr, expected) {
            Some(__addr) => match nix::sys::socket::bind(receiver, &*__addr) {
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
        } == 0)
    );
    (*len.borrow_mut()) = (128usize as libc::socklen_t);
    assert!(
        ({
            let mut __tmp_4 = { (local.as_pointer()).reinterpret_cast::<libcc2rs::Sockaddr>() };
            let mut __tmp_5 = { (len.as_pointer()) };
            match nix::sys::socket::getsockname::<nix::sys::socket::SockaddrStorage>(receiver) {
                Ok(__ss) => {
                    Sockaddr::encode(&__ss, &__tmp_4, &__tmp_5);
                    0
                }
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 0)
    );
    assert!(({ ((*len.borrow()) == expected) }));
    assert!(
        (match Sockaddr::decode(
            &(local.as_pointer()).reinterpret_cast::<libcc2rs::Sockaddr>(),
            ({ (*len.borrow()) })
        ) {
            Some(__addr) => match nix::sys::socket::connect(sender, &*__addr) {
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
        } == 0)
    );
    let peer: Value<libcc2rs::SockaddrStorage> = Rc::new(RefCell::new(Default::default()));
    (*len.borrow_mut()) = (128usize as libc::socklen_t);
    assert!(
        ({
            let mut __tmp_6 = { (peer.as_pointer()).reinterpret_cast::<libcc2rs::Sockaddr>() };
            let mut __tmp_7 = { (len.as_pointer()) };
            match nix::sys::socket::getpeername::<nix::sys::socket::SockaddrStorage>(sender) {
                Ok(__ss) => {
                    Sockaddr::encode(&__ss, &__tmp_6, &__tmp_7);
                    0
                }
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 0)
    );
    assert!(({ ((*len.borrow()) == expected) }));
    assert!(
        ((local.as_pointer())
            .to_any()
            .memcmp(&(peer.as_pointer()).to_any(), (expected as usize))
            == 0)
    );
    let sent: Value<i8> = Rc::new(RefCell::new((('x' as i32) as i8)));
    let received: Value<i8> = Rc::new(RefCell::new(0_i8));
    assert!(
        ({
            let __buf = (sent.as_pointer()).to_any().reinterpret_cast::<u8>();
            match __buf.with_slice(1_usize, |__s| {
                nix::sys::socket::send(
                    sender,
                    __s,
                    nix::sys::socket::MsgFlags::from_bits_truncate(0),
                )
            }) {
                Ok(__n) => __n as isize,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 1_isize)
    );
    let from: Value<libcc2rs::SockaddrStorage> = Rc::new(RefCell::new(Default::default()));
    (*len.borrow_mut()) = (128usize as libc::socklen_t);
    assert!(
        ({
            let mut __tmp_8 = { (from.as_pointer()).reinterpret_cast::<libcc2rs::Sockaddr>() };
            let mut __tmp_9 = { (len.as_pointer()) };
            let __buf = (received.as_pointer()).to_any().reinterpret_cast::<u8>();
            match __buf.with_slice_mut(1_usize, |__s| {
                nix::sys::socket::recvfrom::<nix::sys::socket::SockaddrStorage>(receiver, __s)
            }) {
                Ok((__n, __from)) => {
                    match __from {
                        Some(__ss) => {
                            if !__tmp_8.is_null() {
                                Sockaddr::encode(&__ss, &__tmp_8, &__tmp_9)
                            }
                        }
                        None => {}
                    }
                    __n as isize
                }
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 1_isize)
    );
    assert!(({ ((*len.borrow()) == expected) }));
    assert!(({ (((*received.borrow()) as i32) == ((*sent.borrow()) as i32)) }));
    (*len.borrow_mut()) = (128usize as libc::socklen_t);
    assert!(
        ({
            let mut __tmp_10 = { (local.as_pointer()).reinterpret_cast::<libcc2rs::Sockaddr>() };
            let mut __tmp_11 = { (len.as_pointer()) };
            match nix::sys::socket::getsockname::<nix::sys::socket::SockaddrStorage>(sender) {
                Ok(__ss) => {
                    Sockaddr::encode(&__ss, &__tmp_10, &__tmp_11);
                    0
                }
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 0)
    );
    assert!(({ ((*len.borrow()) == expected) }));
    assert!(
        ((local.as_pointer())
            .to_any()
            .memcmp(&(from.as_pointer()).to_any(), (expected as usize))
            == 0)
    );
    (*received.borrow_mut()) = 0_i8;
    assert!(
        ({
            let __buf = (sent.as_pointer()).to_any().reinterpret_cast::<u8>();
            match __buf.with_slice(1_usize, |__s| {
                nix::sys::socket::send(
                    sender,
                    __s,
                    nix::sys::socket::MsgFlags::from_bits_truncate(0),
                )
            }) {
                Ok(__n) => __n as isize,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 1_isize)
    );
    assert!(
        ({
            let __buf = (received.as_pointer()).to_any().reinterpret_cast::<u8>();
            match __buf.with_slice_mut(1_usize, |__s| {
                nix::sys::socket::recvfrom::<nix::sys::socket::SockaddrStorage>(receiver, __s)
            }) {
                Ok((__n, __from)) => {
                    match __from {
                        Some(__ss) => {
                            if !Ptr::<libcc2rs::Sockaddr>::null().is_null() {
                                Sockaddr::encode(
                                    &__ss,
                                    &Ptr::<libcc2rs::Sockaddr>::null(),
                                    &Ptr::<libc::socklen_t>::null(),
                                )
                            }
                        }
                        None => {}
                    }
                    __n as isize
                }
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 1_isize)
    );
    assert!(({ (((*received.borrow()) as i32) == ((*sent.borrow()) as i32)) }));
    (*received.borrow_mut()) = 0_i8;
    (*len.borrow_mut()) = (77 as libc::socklen_t);
    assert!(
        ({
            let __buf = (sent.as_pointer()).to_any().reinterpret_cast::<u8>();
            match __buf.with_slice(1_usize, |__s| {
                nix::sys::socket::send(
                    sender,
                    __s,
                    nix::sys::socket::MsgFlags::from_bits_truncate(0),
                )
            }) {
                Ok(__n) => __n as isize,
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 1_isize)
    );
    assert!(
        ({
            let mut __tmp_12 = { (len.as_pointer()) };
            let __buf = (received.as_pointer()).to_any().reinterpret_cast::<u8>();
            match __buf.with_slice_mut(1_usize, |__s| {
                nix::sys::socket::recvfrom::<nix::sys::socket::SockaddrStorage>(receiver, __s)
            }) {
                Ok((__n, __from)) => {
                    match __from {
                        Some(__ss) => {
                            if !Ptr::<libcc2rs::Sockaddr>::null().is_null() {
                                Sockaddr::encode(
                                    &__ss,
                                    &Ptr::<libcc2rs::Sockaddr>::null(),
                                    &__tmp_12,
                                )
                            }
                        }
                        None => {}
                    }
                    __n as isize
                }
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 1_isize)
    );
    assert!(({ ((*len.borrow()) == (77 as libc::socklen_t)) }));
    assert!(({ (((*received.borrow()) as i32) == ((*sent.borrow()) as i32)) }));
    assert!((FdRegistry::close(sender) == 0));
    assert!((FdRegistry::close(receiver) == 0));
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let in_: Value<libcc2rs::SockaddrIn> = Rc::new(RefCell::new(Default::default()));
    {
        let mut __tmp_13 = { (in_.as_pointer()).to_any() };
        __tmp_13.memset((0) as u8, 16usize);
        (__tmp_13).clone()
    };
    (*in_.borrow_mut()).sin_family = 2_u16;
    (*in_.borrow_mut()).sin_addr.s_addr = ((u32::to_be(2130706433_u32)) as ::libc::in_addr_t);
    ({
        let _addr: Ptr<libcc2rs::Sockaddr> =
            (in_.as_pointer()).reinterpret_cast::<libcc2rs::Sockaddr>();
        let _expected: libc::socklen_t = (16usize as libc::socklen_t);
        check_name_0(_addr, _expected)
    });
    let in6: Value<libcc2rs::SockaddrIn6> = Rc::new(RefCell::new(Default::default()));
    let mut fd: i32 = {
        let __family = match libc::AF_INET6 {
            ::libc::AF_INET => nix::sys::socket::AddressFamily::Inet,
            ::libc::AF_INET6 => nix::sys::socket::AddressFamily::Inet6,
            ::libc::AF_UNIX => nix::sys::socket::AddressFamily::Unix,
            __d => panic!("socket: unsupported domain {__d}"),
        };
        let __flags = nix::sys::socket::SockFlag::from_bits_truncate(libc::SOCK_DGRAM);
        let __ty = match libc::SOCK_DGRAM & !nix::sys::socket::SockFlag::all().bits() {
            ::libc::SOCK_STREAM => nix::sys::socket::SockType::Stream,
            ::libc::SOCK_DGRAM => nix::sys::socket::SockType::Datagram,
            __t => panic!("socket: unsupported type {__t}"),
        };
        let __proto = match 0 {
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
    };
    assert!((fd >= 0));
    let len: Value<libc::socklen_t> = Rc::new(RefCell::new((28usize as libc::socklen_t)));
    assert!(
        ({
            let mut __tmp_14 = { (in6.as_pointer()).reinterpret_cast::<libcc2rs::Sockaddr>() };
            let mut __tmp_15 = { (len.as_pointer()) };
            match nix::sys::socket::getsockname::<nix::sys::socket::SockaddrStorage>(fd) {
                Ok(__ss) => {
                    Sockaddr::encode(&__ss, &__tmp_14, &__tmp_15);
                    0
                }
                Err(__e) => {
                    libcc2rs::cpp2rust_errno().write(__e as i32);
                    -1
                }
            }
        } == 0)
    );
    assert!(({ (((*len.borrow()) as usize) == 28usize) }));
    assert!((FdRegistry::close(fd) == 0));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
